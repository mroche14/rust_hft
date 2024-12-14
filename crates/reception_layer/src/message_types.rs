//! message_types.rs
//! Defines internal message formats for normalized data (e.g., quotes, trades, order updates).

pub enum MarketMessage {
    Quote {
        symbol: String,
        price: f64,
        size: u64,
        timestamp: u64,
    },
    Trade {
        symbol: String,
        price: f64,
        size: u64,
        timestamp: u64,
    },
    OrderUpdate {
        symbol: String,
        order_id: u64,
        price: f64,
        size: u64,
        timestamp: u64,
    },
}
