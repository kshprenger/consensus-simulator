//! Time representation in DScale simulations.
//!
//! This module defines the `Jiffies` struct, which represents discrete time units
//! in DScale simulations. Jiffies provide a deterministic, integer-based time
//! system that ensures reproducible simulation results.

use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul, Sub},
};

/// A discrete unit of simulation time in DScale.
///
/// `Jiffies` represents time as discrete, integer-based units rather than
/// continuous time. This approach ensures deterministic behavior across
/// simulation runs and eliminates floating-point precision issues that
/// could lead to non-reproducible results.
///
/// The actual duration represented by one jiffy is abstract and depends on
/// your simulation's context. You can think of jiffies as milliseconds,
/// microseconds, or any other time unit that makes sense for your system.
///
/// # Design Philosophy
///
/// - **Deterministic**: Integer arithmetic ensures identical results across runs
/// - **Discrete**: Events happen at specific time points, not continuously
/// - **Abstract**: The real-world duration of a jiffy is context-dependent
/// - **Efficient**: Simple integer operations with no floating-point overhead
///
/// # Usage Patterns
///
/// Jiffies are used throughout DScale for:
/// - Message delivery delays
/// - Timer scheduling
/// - Bandwidth calculations
/// - Simulation time budgets
/// - Progress tracking
///
/// # Examples
///
/// ```rust
/// use dscale::{Jiffies, schedule_timer_after, now};
/// use dscale::helpers::debug_process;
///
/// // Creating jiffies
/// let delay = Jiffies(100);
/// let timeout = Jiffies(5000);
///
/// // Arithmetic operations
/// let total_time = delay + timeout;  // Jiffies(5100)
/// let doubled = delay * 2;           // Not directly supported, use usize * Jiffies
/// let remaining = timeout - delay;   // Jiffies(4900)
///
/// // In a process context
/// fn schedule_work() {
///     let current_time = now();
///     debug_process!("Current time: {}", current_time);
///
///     // Schedule a timer for 1000 jiffies from now
///     schedule_timer_after(Jiffies(1000));
/// }
/// ```
///
/// # Conversion and Display
///
/// Jiffies implement `Display` and `Debug` for easy logging:
///
/// ```rust
/// use dscale::Jiffies;
///
/// let time = Jiffies(12345);
/// println!("{}", time);    // Prints: "Jiffies(12345)"
/// println!("{:?}", time);  // Prints: "12345"
/// ```
#[derive(PartialEq, PartialOrd, Ord, Eq, Copy, Clone, Default)]
pub struct Jiffies(pub usize);

impl Add for Jiffies {
    type Output = Jiffies;

    fn add(self, rhs: Self) -> Self::Output {
        Jiffies(self.0 + rhs.0)
    }
}

impl Sub for Jiffies {
    type Output = Jiffies;

    fn sub(self, rhs: Self) -> Self::Output {
        Jiffies(self.0 - rhs.0)
    }
}

impl AddAssign<Jiffies> for Jiffies {
    fn add_assign(&mut self, rhs: Jiffies) {
        self.0 += rhs.0
    }
}

impl AddAssign<usize> for Jiffies {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs
    }
}

impl Mul<Jiffies> for usize {
    type Output = Self;

    fn mul(self, rhs: Jiffies) -> Self::Output {
        self * rhs.0
    }
}

impl Display for Jiffies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(("Jiffies(".to_string() + &self.0.to_string() + ")").as_str())
    }
}

impl Debug for Jiffies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
