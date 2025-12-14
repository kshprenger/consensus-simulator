use std::cmp::Reverse;

use crate::{process::ProcessId, time::Jiffies};

pub trait Message: Clone {
    fn VirtualSize(&self) -> usize;
}

#[derive(Clone)]
pub struct ProcessStep<M> {
    pub(crate) source: ProcessId,
    pub(crate) dest: ProcessId,
    pub(crate) message: M,
}

#[derive(Clone)]
pub struct RoutedMessage<M> {
    pub(crate) arrival_time: Jiffies,
    pub(crate) step: ProcessStep<M>,
}

impl<M: Message> PartialEq for RoutedMessage<M> {
    fn eq(&self, other: &Self) -> bool {
        self.arrival_time.eq(&other.arrival_time)
    }
}

impl<M: Message> Eq for RoutedMessage<M> {}

impl<M: Message> PartialOrd for RoutedMessage<M> {
    fn ge(&self, other: &Self) -> bool {
        self.arrival_time.ge(&other.arrival_time)
    }
    fn le(&self, other: &Self) -> bool {
        self.arrival_time.le(&other.arrival_time)
    }
    fn gt(&self, other: &Self) -> bool {
        self.arrival_time.gt(&other.arrival_time)
    }
    fn lt(&self, other: &Self) -> bool {
        self.arrival_time.lt(&other.arrival_time)
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.arrival_time.partial_cmp(&other.arrival_time)
    }
}

impl<M: Message> Ord for RoutedMessage<M> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.arrival_time.cmp(&other.arrival_time)
    }
}

pub type TimePriorityMessageQueue<M> = std::collections::BinaryHeap<Reverse<RoutedMessage<M>>>;
