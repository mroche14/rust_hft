# Advanced Course on NUMA Optimization and Memory Management in High-Frequency Trading (HFT)

High-Frequency Trading (HFT) systems rely on cutting-edge hardware and software optimizations to achieve ultra-low latency and high throughput. This course focuses on how Non-Uniform Memory Access (NUMA) and advanced memory management strategies help optimize performance for demanding HFT workloads.

---

## **1. Understanding CPU Architecture**

A deep understanding of CPU architecture is essential to mastering HFT optimization. Here's an overview of key CPU concepts:

### **How a CPU Works**
- **Clock Speed**: Measured in GHz, it determines the number of cycles a CPU can execute per second.
  - **Relevance**: Higher clock speeds can reduce latency in HFT systems.
- **Cycles**: A single iteration of the CPU’s fetch-decode-execute cycle. Multiple instructions may execute per cycle depending on the architecture.
  - **Relevance**: Minimizing cycles per instruction (CPI) is critical for HFT performance.
- **Threads and Processes**:
  - **Thread**: The smallest unit of execution, sharing resources like L1/L2 cache with other threads on the same core.
  - **Process**: An independent unit of execution with its own memory space.
  - **Relevance**: Efficient threading and process allocation ensure maximum CPU utilization.

---

## **2. Memory Hierarchy and Latency**

Modern systems have a hierarchical memory structure. Each level has trade-offs between latency, size, and speed:

| **Memory Type**      | **Latency (Approximate)** | **Size (Order of Magnitude)** | **Relevance in HFT**                          |
|-----------------------|---------------------------|-------------------------------|-----------------------------------------------|
| **Registers**         | ~0.5-1 ns                | ~1 KB                         | Fastest access for immediate computations.    |
| **L1 Cache**          | ~1-4 ns                  | 32-128 KB per core            | Ideal for frequently accessed, small data.    |
| **L2 Cache**          | ~4-14 ns                 | 256 KB-2 MB per core          | Stores slightly larger, less frequently used data. |
| **L3 Cache**          | ~14-30 ns                | 8-64 MB shared per chip       | Shared across cores, useful for inter-core communication. |
| **NUMA-Local DRAM**   | ~60-100 ns               | GBs per NUMA node             | Provides high-speed access to large buffers.  |
| **Remote DRAM**       | ~100-300 ns              | GBs per remote NUMA node      | Significant performance hit; avoid if possible. |
| **Disk (SSD)**        | ~50-100 µs             | TBs                           | Not directly relevant for ultra-low latency operations. |
| **Network Storage**   | ~100-500 µs            | TBs or PBs                    | Used for historical data, not real-time.      |

### **Latency in Context**
Latency refers to the time delay between issuing a memory request and receiving the data. In HFT, minimizing latency ensures that trading decisions are executed before competitors.

---

## **3. NUMA Optimization in HFT Systems**

NUMA (Non-Uniform Memory Access) is a hardware architecture designed to optimize memory access in multi-CPU systems. Each CPU has local memory and faster access to it than to remote memory.

### **NUMA’s Benefits**
1. **Reduced Latency**:
   - By ensuring threads access memory on the same NUMA node, latency is minimized.
2. **Increased Throughput**:
   - Threads can operate independently with local memory, avoiding bottlenecks.
3. **Efficient Scaling**:
   - Large systems distribute workloads across multiple NUMA nodes.

### **How NUMA Solves “Too Big for L1” Buffers**
- NUMA allocates memory close to the CPU core processing it, reducing access latency compared to remote memory.
- While the buffer cannot fit in L1 cache, NUMA-local memory ensures efficient access by:
  - Minimizing contention.
  - Allowing hierarchical cache systems (L1, L2, L3) to operate effectively.

### **NUMA Optimization Techniques**
1. **Thread Affinity**:
   - Pin threads to specific CPU cores to maintain cache locality and reduce cache thrashing.
2. **Memory Affinity**:
   - Allocate buffers in the same NUMA node as the threads processing them.
3. **Load Balancing**:
   - Distribute workloads evenly across NUMA nodes to avoid hotspots.

---

## **4. Advanced Memory Management for HFT**

HFT systems require highly optimized memory strategies to process vast amounts of market data and execute trades within microseconds.

### **Ring Buffers in HFT**
- A **ring buffer** (similar to a circular buffer) is a fixed-size buffer that overwrites old data when full.
  - **Advantages**:
    - Lock-free design for high performance.
    - Efficient memory usage with predictable access patterns.
  - **Relevance**:
    - Used for storing market data feeds and inter-thread communication.

### **NUMA + Ring Buffers**
1. Allocate the ring buffer on a NUMA-local memory bank to reduce latency.
2. Use separate buffers for each NUMA node to minimize contention and ensure local access.

---

## **5. Practical Implementation Example**

### **NUMA-Optimized Workflow for Market Data Processing**
1. **Data Arrival**:
   - Market data arrives via NICs (Network Interface Cards).
   - Each NIC is bound to a specific NUMA node for efficient processing.
2. **Buffer Allocation**:
   - Allocate separate ring buffers for each NIC on the corresponding NUMA node.
3. **Thread Assignment**:
   - Pin threads to cores on the same NUMA node as the NIC and buffer.
4. **Processing Flow**:
   - Thread reads data from the ring buffer.
   - Performs computations (e.g., price updates, risk checks) using L1/L2 cache.
   - Writes results back to the ring buffer or sends orders to the OMS.

### **Mock Data Example**
#### Ring Buffer Configuration
- Size: 1 GB per NUMA node.
- Structure:
  ```rust
  struct MarketData {
      timestamp: u64,
      symbol: [u8; 8],
      price: f64,
      volume: u32,
  }
  ```
#### Simulated Data:
```json
{
  "timestamp": 1638247400000,
  "symbol": "AAPL",
  "price": 150.25,
  "volume": 1000
}
```

---

## **6. Cache Thrashing and Mitigation**

### **What is Cache Thrashing?**
- **Definition**: When multiple threads or processes repeatedly overwrite each other’s data in shared caches (e.g., L3).
- **Impact**:
  - Increased cache misses.
  - Higher latency due to frequent memory accesses.

### **Mitigation Strategies**
1. **Thread Affinity**:
   - Pin threads to specific cores to maintain cache locality.
2. **Data Partitioning**:
   - Divide data across NUMA nodes and cores to minimize contention.
3. **Cache Line Alignment**:
   - Align data structures to cache line boundaries to prevent false sharing.

---

## **Conclusion**
NUMA and advanced memory management strategies are essential for achieving the ultra-low latency and high throughput required in HFT systems. By understanding the memory hierarchy, optimizing data locality, and using lock-free designs like ring buffers, HFT systems can process market data and execute trades with unparalleled efficiency.

This course equips you with the knowledge to design and optimize HFT systems that leverage NUMA architectures, ensuring competitive performance in the high-stakes world of trading.

