use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct ColumnConfig {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub r#type: String,
    #[serde(default)]
    pub delta_encoding: bool,
}

#[derive(Debug, Deserialize)]
pub struct BufferConfig {
    pub num_rows: usize,
    pub num_cols: usize,
    pub columns: Vec<ColumnConfig>,
}

#[derive(Debug, Deserialize)]
pub struct OSOptimizationConfig {
    pub use_huge_pages: bool,
    pub lock_memory: bool,
    pub realtime_priority: i32,
    pub cpu_affinity: usize,
    #[allow(dead_code)]
    pub numa_node: usize,
    #[allow(dead_code)]
    #[serde(default)]
    pub disable_numa: bool, // Default to false if not specified in the config
}

#[derive(Debug, Deserialize)]
pub struct RDMAConfig {
    pub enable: bool,
}

#[derive(Debug, Deserialize)]
pub struct PrefetchingConfig {
    pub enabled: bool,
    pub distance: usize,
}

#[derive(Debug, Deserialize)]
pub struct ProfilingConfig {
    pub enable: bool,
    pub output_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub buffer: BufferConfig,
    pub os_optimization: OSOptimizationConfig,
    pub rdma: RDMAConfig,
    pub prefetching: PrefetchingConfig,
    pub profiling: ProfilingConfig,
}
