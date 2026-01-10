//! Stop-Loss Feature
//!
//! Automatic transaction execution when price thresholds are met.
//! Integrates with oracle price feeds for real-time monitoring.

pub mod manager;
pub mod monitor;

#[cfg(test)]
mod tests;

pub use manager::{StopLossManager, StopLossOrder, StopLossType, StopLossStatus};
pub use monitor::StopLossMonitor;
