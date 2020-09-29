use std::collections::HashSet;
use yew::worker::*;

use super::Toast;

pub struct ToastAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for ToastAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Toast;
    type Output = Toast;

    fn create(link: AgentLink<Self>) -> Self {
        ToastAgent {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
