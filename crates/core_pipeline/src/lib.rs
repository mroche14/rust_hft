//! lib.rs
//! Entry point for the `core_pipeline` crate. Re-exports modules for public use.

pub mod order_book;
pub mod match_engine;
pub mod risk_checks;
pub mod aggregator;
pub mod signal_generator;
pub mod allocators;
pub mod lock_free_queues;
pub mod timing;
