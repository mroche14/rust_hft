// mock_process.rs
use crate::circular_buffer::CircularBuffer;
use crate::profiling::Profiler;
use crate::config::BufferConfig;
use rand::Rng;
use std::time::Duration;
use std::thread;

pub fn generate_mock_data(cbuf: &mut CircularBuffer, buffer_config: &BufferConfig, profiler: &mut Profiler) {
    let num_rows = buffer_config.num_rows;
    let num_cols = buffer_config.num_cols;
    let mut rng = rand::thread_rng();

    loop {
        for _ in 0..num_rows {
            // Generate a random row
            let mut row = Vec::with_capacity(num_cols);
            for _ in 0..num_cols {
                row.push(rng.gen_range(0.0..1_000_000.0));
            }

            // Measure write latency
            let start_t = std::time::Instant::now();
            cbuf.write_row(&row);
            let write_latency = start_t.elapsed().as_nanos() as u64;
            profiler.record_latency(write_latency);
        }

        // Simulate a burst by sleeping between data generation cycles
        thread::sleep(Duration::from_millis(100));
    }
}
