use crate::{
    ipc::{simulator::start, SimulatorChannel},
    log::{debug, error, info, stdio::proxy_stdio, trace, warn, Level, LevelFilter, Record},
    plugin::Plugin,
    protocol::message::{InitializeRequest, Request, Response},
};
use crossbeam_channel::Sender;
use failure::Error;
use ipc_channel::router::ROUTER;
use std::process::{Child, Command};

#[derive(Debug)]
pub struct PluginProcess {
    child: Child,
    channel: SimulatorChannel,
}

impl PluginProcess {
    pub fn new(command: &mut Command, sender: Sender<Record>) -> Result<PluginProcess, Error> {
        debug!("Constructing PluginProcess: {:?}", command);

        let (mut child, mut channel) = start(command, None)?;

        trace!("Forward plugin log channel");
        ROUTER.route_ipc_receiver_to_crossbeam_sender(channel.log().unwrap(), sender.clone());

        // Log piped stdout/stderr
        proxy_stdio(
            Box::new(child.stderr.take().expect("stderr")),
            sender.clone(),
            Level::Error,
        );

        proxy_stdio(
            Box::new(child.stdout.take().expect("stdout")),
            sender,
            Level::Info,
        );

        Ok(PluginProcess { child, channel })
    }

    fn request(&self, request: Request) -> Result<(), Error> {
        self.channel.request.send(request)?;
        Ok(())
    }

    pub fn init<'a>(
        &self,
        downstream: Option<String>,
        upstream: &mut impl Iterator<Item = &'a Plugin>,
    ) {
        trace!("Init: [downstream: {:?}]", downstream);
        self.request(Request::Init(InitializeRequest {
            downstream,
            arb_cmds: None,
            prefix: "".to_string(),
            level: LevelFilter::Trace,
        }))
        .expect("Failed to send init.");
        match self.wait_for_reply() {
            Response::Init(response) => {
                trace!("Got reponse: {:?}", response);
                if let Some(upstream_plugin) = upstream.next() {
                    trace!("Connecting to upstream plugin");
                    upstream_plugin.init(response.upstream, upstream);
                }
            }
            _ => panic!("Bad reply."),
        }
    }
    pub fn abort(&self) {
        self.request(Request::Abort).expect("failed to send abort");
    }

    fn wait_for_reply(&self) -> Response {
        self.channel.response.recv().unwrap()
    }
}

impl Drop for PluginProcess {
    fn drop(&mut self) {
        trace!("Dropping PluginProcess");
        // Wait for child.
        let status = self.child.wait().expect("Child process failed");

        trace!("Child process terminated");
        match status.code() {
            Some(code) => {
                let msg = format!("Exited with status code: {}", code);
                if code > 0 {
                    warn!("{}", msg)
                } else {
                    info!("{}", msg)
                }
            }
            None => error!("Process terminated by signal"),
        };
    }
}
