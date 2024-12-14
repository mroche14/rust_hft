# Huge Page

A **huge page** is a memory management feature provided by modern operating systems that allows processes to allocate and use larger-than-normal memory pages. Traditional memory management divides RAM into small, fixed-size blocks called **pages**, which are typically 4 KB in size. Huge pages increase the page size significantly (e.g., 2 MB or 1 GB), reducing the number of pages needed to map large amounts of memory.

### Key Concepts of Huge Pages

1. **Page Size**:
   - **Normal Page**: Typically 4 KB in most systems.
   - **Huge Page**: Configurable, often 2 MB or 1 GB.

2. **Translation Lookaside Buffer (TLB)**:
   - The TLB is a hardware cache that stores the mapping between virtual memory addresses (used by applications) and physical memory addresses (used by the system).
   - Each entry in the TLB corresponds to a memory page. A huge page covers a much larger memory range, reducing the number of TLB entries needed for the same amount of memory.

3. **Huge Page Benefits**:
   - **Reduced TLB Misses**: Fewer pages mean fewer TLB entries are required, leading to fewer TLB misses and faster memory access.
   - **Lower Overhead**: Managing fewer pages reduces the overhead of maintaining page tables.
   - **Improved Performance**: Particularly for applications with large, contiguous memory allocations (e.g., databases, in-memory caches, HPC workloads).

---

### When and Why to Use Huge Pages

Huge pages are beneficial for applications that:
- Use large, contiguous memory regions.
- Require high memory throughput.
- Suffer from TLB misses due to high memory usage.

Typical use cases include:
- Databases like MySQL, PostgreSQL, and Oracle.
- In-memory key-value stores like Redis and Memcached.
- High-performance computing (HPC) applications.
- Real-time systems requiring deterministic performance.

---

### How to Enable Huge Pages on Linux

#### 1. Configure Huge Pages
Set the number of available huge pages using the `sysctl` command or by writing to `/proc/sys/vm/nr_hugepages`.

Example: Enable 1024 huge pages of 2 MB each (total 2 GB):
```bash
sudo sysctl -w vm.nr_hugepages=1024
```

To make this persistent across reboots, add it to `/etc/sysctl.conf`:
```
vm.nr_hugepages=1024
```

#### 2. Check Huge Page Availability
Use the following command to see current huge page usage:
```bash
cat /proc/meminfo | grep Huge
```

#### 3. Allocate Memory with Huge Pages
Applications can request huge pages explicitly by:
- Using `mmap` with the `MAP_HUGETLB` flag.
- Linking to libraries like `libhugetlbfs` that simplify huge page allocation.
- Configuring huge page support in the application (e.g., many databases and in-memory caches provide settings to enable huge pages).

---

### Drawbacks of Huge Pages

1. **Increased Memory Fragmentation**:
   - Huge pages require contiguous blocks of memory. Allocating huge pages may fail if the system's memory is fragmented.

2. **No On-Demand Paging**:
   - Huge pages are pre-allocated and pinned in memory. They cannot be swapped out to disk, which may reduce the available memory for other processes.

3. **Configuration Overhead**:
   - Setting up huge pages requires administrative privileges and system configuration.

4. **Limited Compatibility**:
   - Some applications and workloads may not benefit from huge pages or require modification to take advantage of them.

---

### Enabling Huge Pages in the Circular Buffer

In the **circular_shared** crate, huge pages can be enabled to optimize memory access for high-performance workloads. The builder provides a `use_hugepages` option:

```rust
let buf = CircularBufferBuilder::new(100, columns)
    .use_hugepages(true) // Enable huge pages
    .build().unwrap();
```

Ensure the system has huge pages configured before running the application. If the application attempts to use huge pages without sufficient pre-allocated pages, the memory mapping may fail.

---

### Summary

- Huge pages increase the size of memory pages (e.g., from 4 KB to 2 MB or 1 GB).
- Benefits include reduced TLB misses, lower page table overhead, and improved performance for large memory allocations.
- Configuring huge pages requires administrative privileges and careful tuning of system settings.
- They are particularly useful for applications with high memory throughput or large, contiguous memory requirements.