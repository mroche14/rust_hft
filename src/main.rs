//! main.rs
//! This is the entry point of the entire application. It sets up the runtime environment,
//! loads configuration, applies CPU pinning, and initializes all pipelines (reception, core, storage, analytics).
//! It then starts the event loop or dispatcher that coordinates data flow.

mod config;
mod sys_pinning;
mod runtime;

use crate::config::Config;
use crate::sys_pinning::apply_cpu_pinning;
use crate::runtime::RuntimeManager;

fn main() {
    // Load configuration from file or environment
    let config = Config::load().expect("Failed to load config");

    // Apply CPU pinning for deterministic performance
    apply_cpu_pinning(&config);

    // Initialize the runtime (threads, channels, pipelines)
    let mut runtime = RuntimeManager::new(config);

    // Start the system
    runtime.start();

    // The system might run indefinitely until a shutdown signal is received
    runtime.wait_for_shutdown();
}
