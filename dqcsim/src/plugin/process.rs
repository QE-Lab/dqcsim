use crate::{
    ipc::{simulator::start, SimulatorChannel},
    log::{
        debug, error, fatal, info, router::route, stdio::proxy_stdio, trace, warn, Loglevel,
        LoglevelFilter, Record,
    },
    plugin::Plugin,
    protocol::message::{InitializeRequest, Request, Response},
};
use crossbeam_channel::Sender;
use failure::{bail, Error};
use std::process::{Child, Command, ExitStatus};

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
        route(channel.log().unwrap(), sender.clone());

        // Log piped stdout/stderr
        proxy_stdio(
            Box::new(child.stderr.take().expect("stderr")),
            sender.clone(),
            Loglevel::Error,
        );

        proxy_stdio(
            Box::new(child.stdout.take().expect("stdout")),
            sender,
            Loglevel::Info,
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
    ) -> Result<(), Error> {
        trace!("Init: [downstream: {:?}]", downstream);
        self.request(Request::Init(InitializeRequest {
            downstream,
            arb_cmds: None,
            prefix: "".to_string(),
            level: LoglevelFilter::Trace,
        }))?;
        trace!("Waiting for init reply.");
        match self.wait_for_reply() {
            Response::Init(response) => {
                trace!("Got reponse: {:?}", response);
                if let Some(upstream_plugin) = upstream.next() {
                    trace!("Connecting to upstream plugin");
                    upstream_plugin.init(response.upstream, upstream)?;
                }
                Ok(())
            }
            _ => {
                fatal!("Bad reply from plugin");
                bail!("Bad reply from plugin")
            }
        }
    }
    pub fn abort(&mut self, graceful: bool) -> Result<Option<ExitStatus>, Error> {
        if graceful {
            self.request(Request::Abort)?;
        } else {
            self.child.kill()?;
        }
        Ok(self.child.try_wait()?)
    }

    fn wait_for_reply(&self) -> Response {
        self.channel.response.recv().unwrap()
    }
}

impl Drop for PluginProcess {
    fn drop(&mut self) {
        trace!("Dropping PluginProcess");

        // Wait for child.
        // TODO: try_wait?
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
