// Provides a profiling mechanism to record latencies and dump them to a JSON file.

use crate::config::ProfilingConfig;
use std::time::Instant;
use std::fs::File;
use std::io::Write;
use serde::Serialize;

#[derive(Serialize)]
struct ProfilingData {
    latency_ns: Vec<u64>,
    timestamp: u64,
}

pub struct Profiler {
    enabled: bool,
    start: Instant,
    data: ProfilingData,
    output_path: String,
}

impl Profiler {
    pub fn new(cfg: &ProfilingConfig) -> Self {
        Profiler {
            enabled: cfg.enable,
            start: Instant::now(),
            data: ProfilingData {
                latency_ns: vec![],
                timestamp: 0,
            },
            output_path: cfg.output_path.clone(),
        }
    }

    pub fn record_latency(&mut self, latency_ns: u64) {
        if self.enabled {
            self.data.latency_ns.push(latency_ns);
        }
    }

    pub fn finalize(&mut self) {
        if self.enabled {
            self.data.timestamp = self.start.elapsed().as_nanos() as u64;
            let json = serde_json::to_string(&self.data).unwrap();
            let mut f = File::create(&self.output_path).unwrap();
            f.write_all(json.as_bytes()).unwrap();
        }
    }
}

impl Drop for Profiler {
    fn drop(&mut self) {
        self.finalize();
    }
}
