//! integration_tests.rs
//! End-to-end tests: feed mock data into reception layer, ensure core pipeline produces expected signals.

#[test]
fn test_integration_flow() {
    // Pseudocode:
    // 1. Start mock data gen
    // 2. Feed into reception -> decode -> order book -> aggregator -> signal generator
    // 3. Assert expected signals
}

