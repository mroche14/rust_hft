//! risk_checks.rs
//! Validates orders and signals against risk parameters.
//!
//! # Purpose
//! - Prevent placing orders that exceed pre-defined limits.
//! - This must be done quickly to not impact latency.

pub struct RiskChecker {
    max_order_size: u64,
}

impl RiskChecker {
    pub fn new(max_order_size: u64) -> Self {
        RiskChecker { max_order_size }
    }

    pub fn check_order_size(&self, size: u64) -> bool {
        size <= self.max_order_size
    }
}
