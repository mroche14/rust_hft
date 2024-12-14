//! signal_generator.rs
//! Converts aggregator metrics into actionable signals (e.g., trading signals).
//!
//! # Idea
//! If EMA surpasses a certain threshold, generate a "BUY" signal, etc.

pub struct SignalGenerator {
    buy_threshold: f64,
    sell_threshold: f64,
}

impl SignalGenerator {
    pub fn new(buy_threshold: f64, sell_threshold: f64) -> Self {
        SignalGenerator {
            buy_threshold,
            sell_threshold,
        }
    }

    pub fn generate_signal(&self, metric: f64) -> Option<&'static str> {
        if metric > self.buy_threshold {
            Some("BUY")
        } else if metric < self.sell_threshold {
            Some("SELL")
        } else {
            None
        }
    }
}
