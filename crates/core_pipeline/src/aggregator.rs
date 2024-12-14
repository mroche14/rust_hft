//! aggregator.rs
//! Computes short-term financial metrics like EMA and could be extended with regression or volatility metrics.
//!
//! # Example
//! - Update aggregator with each new trade price
//! - Recalculate EMA in O(1)
//! - Optionally compute linear regression over last N data points.

pub struct Aggregator {
    window_size: usize,
    prices: Vec<f64>,
    current_ema: f64,
}

impl Aggregator {
    pub fn new(window_size: usize) -> Self {
        Aggregator {
            window_size,
            prices: Vec::with_capacity(window_size),
            current_ema: 0.0,
        }
    }

    pub fn update_price(&mut self, price: f64) {
        // Add the new price, remove old if at capacity
        if self.prices.len() == self.window_size {
            self.prices.remove(0);
        }
        self.prices.push(price);

        // Compute EMA incrementally
        let alpha = 2.0 / (self.window_size as f64 + 1.0);
        self.current_ema = if self.prices.len() == 1 {
            // Initialize EMA with the first price
            price
        } else {
            alpha * price + (1.0 - alpha) * self.current_ema
        }
    }

    /// Return the current EMA value
    pub fn ema(&self) -> f64 {
        self.current_ema
    }

    // Future: implement regression or other statistical measures here.
}
