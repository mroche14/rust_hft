//! config.rs
//! Responsible for loading and validating configuration parameters: exchange endpoints, instruments,
//! CPU affinity, aggregator parameters, etc.

use std::error::Error;

#[derive(Debug)]
pub struct Config {
    /// The exchange endpoints to connect to
    pub exchange_endpoints: Vec<String>,
    /// CPU cores assigned to hot paths
    pub cpu_cores: Vec<usize>,
    /// Aggregation parameters (e.g., EMA window size)
    pub ema_window: usize,
    /// Whether to enable mock data generation for testing
    pub use_mock_data: bool,
}

impl Config {
    /// Load configuration from a file or environment.
    pub fn load() -> Result<Self, Box<dyn Error>> {
        // In real code, parse JSON/YAML or env variables.
        // Here, we mock a configuration.
        Ok(Config {
            exchange_endpoints: vec!["udp://127.0.0.1:5000".into()],
            cpu_cores: vec![0, 1],    // Pin main threads to cores 0 and 1
            ema_window: 20,          // Example EMA window size
            use_mock_data: true,
        })
    }

    /// Validate configuration parameters for correctness
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.exchange_endpoints.is_empty() {
            return Err("No exchange endpoints specified");
        }
        if self.cpu_cores.is_empty() {
            return Err("No CPU cores specified");
        }
        Ok(())
    }
}
