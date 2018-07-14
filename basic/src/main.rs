extern crate riker;
extern crate riker_default;
#[macro_use]
extern crate log;

use std::time::Duration;
use riker::actors::*;
use riker_default::DefaultModel;

struct MyActor;

// implement the Actor trait
impl Actor for MyActor {
    type Msg = String;

    fn receive(&mut self,
                _ctx: &Context<Self::Msg>,
                msg: Self::Msg,
                _sender: Option<ActorRef<Self::Msg>>) {

        debug!("Received: {}", msg);
    }
}

// provide factory and props functions
impl MyActor {
    fn actor() -> BoxActor<String> {
        Box::new(MyActor)
    }

    fn props() -> BoxActorProd<String> {
        Props::new(Box::new(MyActor::actor))
    }
}

// start the system and create an actor
fn main() {
    let model: DefaultModel<String> = DefaultModel::new();
    let sys = ActorSystem::new(&model).unwrap();

    let props = MyActor::props();
    let my_actor = sys.actor_of(props, "my-actor").unwrap();

    my_actor.tell("Hello my actor!".to_string(), None);

    std::thread::sleep(Duration::from_millis(500));
}