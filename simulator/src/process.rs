use crate::communication::{Destination, Event, Message};
use std::collections::HashSet;

pub type ProcessId = usize;

pub trait ProcessHandle {
    fn init(&mut self); // Should schedule some initial events
    fn on_event(&mut self, m: Event) -> HashSet<(Destination, Message)>;
}
