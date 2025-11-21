use crate::{
    communication::{Destination, Event, EventId, EventType},
    simulation_handle::with_sim,
};

pub type Jiffies = usize;

/// Returns associated with this timeout EventId.
/// This will allow process to cancel it calling reset_timeout.
pub fn schedule_timeout(after: Jiffies) -> EventId {
    with_sim(|sim| sim.submit_event_after(EventType::Timeout, Destination::SendSelf, after))
}

pub fn reset_timeout(timeout_id: EventId) {
    with_sim(|sim| {
        sim.cancel_event(&Event {
            id: timeout_id,
            event_type: EventType::Timeout,
        })
    });
}
