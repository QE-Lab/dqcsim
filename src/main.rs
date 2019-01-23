use env_logger::{Builder, Env};
use futures::executor::block_on;
use log::debug;
use riker::actors::*;
use riker_default::DefaultModel;
use riker_patterns::ask::ask;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    /// Operators configuration
    #[structopt(
        short = "o",
        long = "operator",
        raw(required = "false", min_values = "0")
    )]
    operators: Vec<String>,
}

struct EchoActor;

impl EchoActor {
    fn new() -> BoxActor<String> {
        Box::new(EchoActor)
    }
}

impl Actor for EchoActor {
    type Msg = String;
    fn pre_start(&mut self, ctx: &Context<Self::Msg>) {
        debug!("starting {:?}", ctx.myself());
    }

    fn post_start(&mut self, ctx: &Context<Self::Msg>) {
        debug!("started {:?}", ctx.myself());
    }
    fn post_stop(&mut self) {
        debug!("stopped");
    }

    fn receive(
        &mut self,
        ctx: &Context<Self::Msg>,
        msg: Self::Msg,
        sender: Option<ActorRef<Self::Msg>>,
    ) {
        sender
            .try_tell(String::from("Pong"), Some(ctx.myself()))
            .unwrap();
    }
    fn system_receive(
        &mut self,
        ctx: &Context<Self::Msg>,
        msg: SystemMsg<Self::Msg>,
        sender: Option<ActorRef<Self::Msg>>,
    ) {
        debug!("{:?}", msg);
    }
}

fn main() -> Result<(), ()> {
    Builder::from_env(Env::default().default_filter_or("debug")).init();

    let opt = Opt::from_args();
    // dbg!(&opt);

    let model: DefaultModel<String> = DefaultModel::new();
    let sys = ActorSystem::new(&model).unwrap();

    let props = Props::new(Box::new(EchoActor::new));
    let actor = sys.actor_of(props, "me").unwrap();

    let msg = "Ping".to_string();
    debug!("Sending Ping.");
    let res = ask(&sys, &actor, msg.clone());
    let res = block_on(res);
    debug!("{}", res);

    debug!("Shutting down system after {}s uptime", sys.uptime());
    block_on(sys.shutdown());
    debug!("System down.");
    Ok(())
}
