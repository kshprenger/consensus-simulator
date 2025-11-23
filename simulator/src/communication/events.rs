use priority_queue::PriorityQueue;

use crate::{process::ProcessId, time::Jiffies};

pub type EventId = usize;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub(crate) struct Event {
    pub id: EventId,
    pub event_type: EventType,
}

impl Event {
    pub(crate) fn size(&self) -> usize {
        size_of::<EventId>()
            + match &self.event_type {
                EventType::Timeout => 0,
                EventType::Message(msg) => size_of::<ProcessId>() + msg.payload.len(),
            }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub(crate) enum EventType {
    Timeout,
    Message(Message),
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub enum Destination {
    Broadcast,
    SendSelf,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Message {
    source: ProcessId,
    payload: bytes::Bytes,
}

/// (Jiffies, Event) <=> At speciffied timestamp event will be delivered
pub type TimePriorityEventQueue = PriorityQueue<Event, Jiffies>;
