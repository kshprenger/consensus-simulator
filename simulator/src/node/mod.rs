use crate::communication::Message;
use std::collections::HashSet;

pub trait Node {
    fn on_message(&mut self, m: Message) -> HashSet<Message>;
    fn tick(&mut self) -> HashSet<Message>;
}
