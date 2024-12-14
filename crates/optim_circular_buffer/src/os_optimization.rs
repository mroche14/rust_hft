// os_optimization.rs
use crate::config::OSOptimizationConfig;
use libc::{mlock, sched_setaffinity, sched_param, sched_setscheduler, CPU_SET, cpu_set_t, SCHED_FIFO};
use std::fs::File;
use std::io::Write;
use std::mem;
// use numas::NodeMask; // Update the import

// Apply CPU affinity to bind the process to a specific CPU core.
#[inline]
fn set_cpu_affinity(cpu_id: usize) -> Result<(), String> {
    unsafe {
        let mut cpu_set: cpu_set_t = mem::zeroed();
        CPU_SET(cpu_id, &mut cpu_set);
        let result = sched_setaffinity(0, mem::size_of::<cpu_set_t>(), &cpu_set);
        if result != 0 {
            return Err("Failed to set CPU affinity".into());
        }
    }
    Ok(())
}

// Set real-time scheduling to reduce latency due to scheduling jitter.
#[inline]
fn set_realtime_priority(priority: i32) -> Result<(), String> {
    let param = sched_param { sched_priority: priority };
    let result = unsafe { sched_setscheduler(0, SCHED_FIFO, &param) };
    if result != 0 {
        return Err("Failed to set real-time priority".into());
    }
    Ok(())
}

// Lock all current and future pages into RAM, preventing paging.
#[inline]
fn lock_memory() -> Result<(), String> {
    let ret = unsafe { mlock(std::ptr::null(), libc::RLIM64_INFINITY as usize) };
    if ret != 0 {
        return Err("Failed to lock memory into RAM".into());
    }
    Ok(())
}

// Enable transparent huge pages if requested.
#[inline]
fn enable_huge_pages() -> Result<(), String> {
    let mut file = File::create("/sys/kernel/mm/transparent_hugepage/enabled")
        .map_err(|_| "Failed to open huge pages configuration file. Are you running as root?".to_string())?;
    file.write_all(b"always").map_err(|_| "Failed to enable huge pages".to_string())?;
    Ok(())
}

// Bind to a specific NUMA node to reduce cross-NUMA memory accesses.
// #[inline]
// fn set_numa_node(node: usize) -> Result<(), String> {
//     let numa = NodeMask::new(); // Create a NodeMask instead of Numa

//     if node >= numa.num_nodes() {
//         return Err("NUMA node out of range".into());
//     }

//     let mut mask = NodeMask::new();
//     mask.set(node);
//     numa.set_membind(&mask).map_err(|_| "Failed to set NUMA node policy. Ensure you have permissions and NUMA is enabled.".to_string())
// }

pub fn apply_os_optimizations(cfg: &OSOptimizationConfig) -> Result<(), String> {
    if cfg.use_huge_pages {
        enable_huge_pages()?;
    }

    if cfg.lock_memory {
        lock_memory()?;
    }

    set_cpu_affinity(cfg.cpu_affinity)?;

    #[cfg(feature = "numa")]
    {
        if !cfg.disable_numa {
            numa_support::set_numa_node(cfg.numa_node)?;
        } else {
            println!("NUMA optimizations are disabled.");
        }
    }

    set_realtime_priority(cfg.realtime_priority)?;

    Ok(())
}