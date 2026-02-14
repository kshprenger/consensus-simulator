//! Global simulation clock functionality.
//!
//! This module provides access to the current simulation time through a thread-local
//! storage mechanism. The clock is managed internally by the simulation engine and
//! provides deterministic time progression for all processes.

use std::cell::Cell;

use log::debug;

use crate::Jiffies;

thread_local! {
    pub(crate) static CLOCK: Cell<Jiffies> = Cell::new(Jiffies(0))
}

pub(crate) fn drop_clock() {
    CLOCK.take();
}

pub(crate) fn fast_forward_clock(future: Jiffies) {
    let present = CLOCK.replace(future);
    debug_assert!(present <= future, "Future < Present");
    debug!("Global time now: {future}");
}

/// Returns the current simulation time.
///
/// This function provides access to the global simulation clock, which represents
/// the current time in the simulation. Time is measured in [`Jiffies`], which are
/// the basic unit of time in the DScale simulation framework.
///
/// # Context
///
/// This function can be called from within any process context during simulation
/// execution, including within [`ProcessHandle`] implementations and timer callbacks.
///
/// [`ProcessHandle`]: crate::ProcessHandle
///
/// # Examples
///
/// ```rust
/// use dscale::{now, ProcessHandle, MessagePtr, ProcessId, TimerId};
/// use dscale::helpers::debug_process;
///
/// struct MyProcess;
///
/// impl ProcessHandle for MyProcess {
///     fn start(&mut self) {
///         debug_process!("Process started at time: {}", now());
///     }
///
///     fn on_message(&mut self, from: ProcessId, message: MessagePtr) {
///         debug_process!("Received message at time: {}", now());
///     }
///
///     fn on_timer(&mut self, id: TimerId) {
///         debug_process!("Timer fired at time: {}", now());
///     }
/// }
/// ```
///
/// # Returns
///
/// The current simulation time as [`Jiffies`].
pub fn now() -> Jiffies {
    CLOCK.get()
}
