use std::collections::HashMap;

use crate::{
    communication::{Event, TimePriorityEventQueue},
    process::ProcessId,
    time::Jiffies,
};

#[derive(Clone, Copy)]
pub(crate) enum BandwidthType {
    Unbounded,
    Bounded(usize), // Bytes per Jiffy
}

pub(crate) struct NetworkBoundedQueue {
    bandwidth: usize,
    total_passed: usize,
    queue: TimePriorityEventQueue,
}

impl NetworkBoundedQueue {
    pub(crate) fn new(bandwidth_type: BandwidthType) -> Self {
        let bandwidth = match bandwidth_type {
            BandwidthType::Unbounded => usize::MAX,
            BandwidthType::Bounded(bound) => bound,
        };

        Self {
            bandwidth,
            total_passed: 0,
            queue: TimePriorityEventQueue::new(),
        }
    }

    pub(crate) fn push(&mut self, event: Event, should_arrive_at: Jiffies) {
        self.queue.push(event, should_arrive_at);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub(crate) fn peek(&self) -> Option<(&Event, &Jiffies)> {
        self.queue.peek()
    }

    pub(crate) fn remove(&mut self, event: &Event) {
        self.queue.remove(event);
    }

    pub(crate) fn try_pop(&mut self, current_time: Jiffies) -> Option<Event> {
        match self.queue.peek() {
            None => None,
            Some((event, _)) => {
                if self.total_passed + event.size() > self.bandwidth * current_time {
                    None
                } else {
                    self.total_passed += event.size();
                    Some(self.queue.pop().unwrap().0)
                }
            }
        }
    }
}
