#[cfg(feature = "numa")]
mod numa_support {
    use libc::c_ulong;

    extern "C" {
        pub fn set_mempolicy(mode: i32, nodemask: *const c_ulong, maxnode: usize) -> i32;
    }

    const MPOL_BIND: i32 = 2; // Memory policy for binding memory allocations

    pub fn set_numa_node(node: usize) -> Result<(), String> {
        let maxnode = 1024; // Maximum number of nodes (adjust as necessary)
        let mut nodemask = vec![0u64; (maxnode + 63) / 64];
        let word_index = node / 64;
        let bit_index = node % 64;

        if word_index >= nodemask.len() {
            return Err("NUMA node out of range".into());
        }

        nodemask[word_index] |= 1 << bit_index;

        let ret = unsafe {
            set_mempolicy(
                MPOL_BIND,
                nodemask.as_ptr() as *const c_ulong,
                nodemask.len() * 64,
            )
        };

        if ret != 0 {
            return Err("Failed to set NUMA node policy. Ensure you have permissions and NUMA is enabled.".into());
        }
        Ok(())
    }
}