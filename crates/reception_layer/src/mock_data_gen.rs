//! mock_data_gen.rs
//! Generates synthetic market data for testing. Can produce random prices, or replay from a file.

pub struct MockDataGenerator {
    // Could store rng, scenarios, replay files
}

impl MockDataGenerator {
    pub fn new() -> Self {
        MockDataGenerator {}
    }

    /// Generate a mock data packet simulating a quote or trade.
    pub fn generate(&mut self) -> Vec<u8> {
        // Pseudocode:
        // Create a fake binary message representing a quote/trade.
        vec![0x01, 0x02, 0x03] // stub
    }
}
