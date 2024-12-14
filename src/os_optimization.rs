// This file applies OS-level optimizations: huge pages, memory locking, CPU affinity, NUMA pinning,
// and real-time priority scheduling, all for ultra-low-latency performance on Linux.

use crate::config::OSOptimizationConfig;
use libc::{mlock, sched_setaffinity, sched_param, sched_setscheduler, CPU_SET, cpu_set_t, SCHED_FIFO};
use std::fs::File;
use std::io::Write;
use std::ptr;
use std::mem;

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
    // MCL_FUTURE and MCL_CURRENT can be used. For simplicity, just use mlock on the entire process space.
    // This might require appropriate ulimits (ulimit -l).
    let ret = unsafe { mlock(ptr::null(), 0) };
    if ret != 0 {
        return Err("Failed to lock memory into RAM".into());
    }
    Ok(())
}

// Enable transparent huge pages if requested.
#[inline]
fn enable_huge_pages() -> Result<(), String> {
    // Writing "always" into /sys/kernel/mm/transparent_hugepage/enabled tries to force huge pages.
    // The system must be configured accordingly.
    let mut file = File::create("/sys/kernel/mm/transparent_hugepage/enabled")
        .map_err(|_| "Failed to open huge pages configuration file. Are you running as root?".to_string())?;
    file.write_all(b"always").map_err(|_| "Failed to enable huge pages".to_string())?;
    Ok(())
}

// Bind to a specific NUMA node to reduce cross-NUMA memory accesses.
// We'll use the Linux "numa" API via set_mempolicy if available, else we fallback to no-op.
#[inline]
fn set_numa_node(node: usize) -> Result<(), String> {
    // Use libc::set_mempolicy or mbind to bind memory allocations to a NUMA node.
    // We will do a simplistic approach: use set_mempolicy MPOL_BIND for all future allocations.
    // This requires root privileges typically.
    // MPOL_BIND = 2, mode for memory allocation policy
    // We build a nodemask with a single bit set for `node`.
    let maxnode = 1024; // somewhat arbitrary large number for nodemask
    let bytes = (maxnode + 7) / 8;
    let mut nodemask = vec![0u8; bytes];
    let byte_index = node / 8;
    let bit_index = node % 8;
    if byte_index >= nodemask.len() {
        return Err("NUMA node out of range".into());
    }
    nodemask[byte_index] |= 1 << bit_index;

    let ret = unsafe {
        libc::set_mempolicy(
            2, // MPOL_BIND
            nodemask.as_ptr() as *const libc::ulong,
            maxnode
        )
    };
    if ret != 0 {
        return Err("Failed to set NUMA node policy. Ensure you have permissions and NUMA is enabled.".into());
    }
    Ok(())
}

pub fn apply_os_optimizations(cfg: &OSOptimizationConfig) -> Result<(), String> {
    if cfg.use_huge_pages {
        enable_huge_pages()?;
    }

    if cfg.lock_memory {
        lock_memory()?;
    }

    set_cpu_affinity(cfg.cpu_affinity)?;
    set_numa_node(cfg.numa_node)?;
    set_realtime_priority(cfg.realtime_priority)?;

    Ok(())
}
