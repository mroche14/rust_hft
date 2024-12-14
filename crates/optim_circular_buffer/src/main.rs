use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::fs::File;
use std::io::Read;
use std::mem;
use libc::{mmap, MAP_SHARED, MAP_ANONYMOUS, MAP_HUGETLB, PROT_READ, PROT_WRITE};
use std::ptr;
use std::thread;
use std::time::Duration;

// Add the missing dependency
extern crate ctrlc;

mod config;
mod os_optimization;
mod rdma;
mod delta_encoding;
mod prefetch;
mod profiling;
mod circular_buffer;
mod mock_process;

use crate::config::Config;
use crate::os_optimization::apply_os_optimizations;
use crate::profiling::Profiler;
use crate::rdma::setup_rdma;
use crate::circular_buffer::CircularBuffer;

fn main() {
    // ---------------------------------------------------------------------
    // Load config
    // ---------------------------------------------------------------------
    let mut file = File::open("config.json").expect("Failed to open config.json");
    let mut json_str = String::new();
    file.read_to_string(&mut json_str).expect("Failed to read config.json");
    let cfg: Config = serde_json::from_str(&json_str).expect("Invalid config JSON");

    // ---------------------------------------------------------------------
    // Apply OS-level optimizations
    // ---------------------------------------------------------------------
    apply_os_optimizations(&cfg.os_optimization).expect("Failed to apply OS optimizations");

    // ---------------------------------------------------------------------
    // Compute total size and allocate buffer memory (with huge pages if requested)
    // ---------------------------------------------------------------------
    let row_size = cfg.buffer.num_cols * mem::size_of::<f64>();
    let total_size = mem::size_of::<circular_buffer::SharedHeader>() + (cfg.buffer.num_rows * row_size);

    let mut map_flags = MAP_ANONYMOUS | MAP_SHARED;
    if cfg.os_optimization.use_huge_pages {
        map_flags |= MAP_HUGETLB;
    }

    unsafe {
        let ptr = mmap(ptr::null_mut(), total_size, PROT_READ | PROT_WRITE, map_flags, -1, 0);
        if ptr == libc::MAP_FAILED {
            panic!("Failed to mmap shared memory region with huge pages");
        }

        // ---------------------------------------------------------------------
        // Setup RDMA if needed
        // ---------------------------------------------------------------------
        let _rdma_handle = setup_rdma(&cfg.rdma, ptr as *mut u8, total_size)
            .expect("Failed to setup RDMA");

        // ---------------------------------------------------------------------
        // Initialize the circular buffer
        // ---------------------------------------------------------------------
        let cbuf = Arc::new(Mutex::new(CircularBuffer::new(
            ptr as *mut u8,
            &cfg.buffer,
            cfg.buffer.columns.clone(),
            cfg.prefetching.enabled,
            cfg.prefetching.distance,
        )));

        // ---------------------------------------------------------------------
        // Initialize profiler
        // ---------------------------------------------------------------------
        let profiler = Arc::new(Mutex::new(Profiler::new(&cfg.profiling)));

        // ---------------------------------------------------------------------
        // Graceful termination flag
        // ---------------------------------------------------------------------
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&running);

        ctrlc::set_handler(move || {
            println!("\nGraceful shutdown initiated.");
            running_clone.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");

        // ---------------------------------------------------------------------
        // Start Mock Process
        // ---------------------------------------------------------------------
        let cbuf_clone = Arc::clone(&cbuf);
        let profiler_clone = Arc::clone(&profiler);
        let running_clone = Arc::clone(&running);

        let mock_thread = thread::spawn(move || {
            let mut cbuf = cbuf_clone.lock().unwrap();
            let mut profiler = profiler_clone.lock().unwrap();
            mock_process::generate_mock_data(&mut cbuf, &cfg.buffer, &mut profiler);
        });

        // ---------------------------------------------------------------------
        // Monitor and Wait for Mock Process
        // ---------------------------------------------------------------------
        while running.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_secs(1));
        }

        // ---------------------------------------------------------------------
        // Clean up
        // ---------------------------------------------------------------------
        mock_thread.join().expect("Mock process thread panicked");
        let mut profiler = profiler.lock().unwrap();
        profiler.finalize();
        println!("Profiler results saved. Exiting gracefully.");
    }
}