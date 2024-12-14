# NUMA and High-Frequency Trading: A Comprehensive Course

## **Introduction to NUMA Architectures**

### **History and Evolution**

#### **Early Computer Architectures: UMA**

In the early days of computing, multiprocessor systems adhered to a Uniform Memory Access (UMA) model. In UMA:

- **Memory Uniformity**: All processors shared the same memory with equal access times.
- **Simplicity**: The design was straightforward, suitable for small-scale systems.
- **Scalability Issues**: As the number of processors increased, contention for memory bandwidth led to significant bottlenecks.

#### **Emergence of NUMA**

To address UMA's limitations, Non-Uniform Memory Access (NUMA) was introduced:

- **Localized Memory Access**: Each processor or group of processors (a "node") has its own local memory.
- **Reduced Contention**: Processors primarily access their local memory, minimizing competition for shared resources.
- **Scalable Performance**: NUMA systems scale better with additional processors, making them ideal for high-performance applications.

### **NUMA in Modern Hardware**

- **NUMA Nodes**: Modern systems divide processors and memory into nodes interconnected by high-speed links.
- **Processor Interconnects**: Technologies like Intel’s QPI (QuickPath Interconnect) and AMD’s Infinity Fabric connect NUMA nodes.
- **NUMA in Multi-Socket Systems**: Widely used in data centers and high-performance computing to optimize workloads.

---

## **NUMA's Role in High-Frequency Trading (HFT)**

### **Latency Sensitivity in HFT**

High-Frequency Trading systems operate under stringent latency requirements:

- **Sub-Microsecond Response**: A delay of even a few microseconds can result in missed trading opportunities.
- **Deterministic Performance**: Consistent latency is as critical as low latency itself.

### **NUMA's Contributions**

- **Local Memory Access**: NUMA reduces memory access latency by prioritizing local memory.
- **Throughput Optimization**: NUMA-aware designs ensure maximum data throughput.
- **CPU Affinity**: NUMA allows binding processes to CPUs within the same node for faster data access.

### **Challenges of NUMA in HFT**

- **Memory Contention**: Cross-node memory access introduces latency.
- **Thread Scheduling**: Ensuring threads stay on the same node to minimize performance penalties.
- **Data Placement**: Misaligned memory allocation can negate NUMA’s benefits.

---

## **NUMA Architecture Deep Dive**

### **Components of NUMA**

1. **Nodes**:

   - **Processors**: Each node contains one or more CPUs.
   - **Local Memory**: Memory directly attached to the CPUs within the node, typically ranging from several GB (e.g., 64 GB) to several TB (e.g., 1-2 TB) depending on system configuration.
   - **Interconnects**: Links between nodes for cross-node communication.

2. **Memory Access Types**:

   - **Local Memory Access**: Fast access to memory within the same node.
   - **Remote Memory Access**: Slower access to memory located in another node.

3. **NUMA Topology**:

   - **Symmetric NUMA**: Equal number of CPUs and memory per node.
   - **Asymmetric NUMA**: Uneven distribution of CPUs and memory across nodes.

### **NUMA Nodes in Operating Systems**

- **Linux NUMA Tools**:

  - `numactl`: Control NUMA policy for processes and memory allocation.
  - `numastat`: Monitor NUMA-related performance metrics.

- **Windows NUMA Support**:

  - NUMA topology is exposed through Windows APIs for custom optimizations.

---

## **NUMA Optimization Techniques for HFT**

### **Thread and Process Affinity**

- **Binding Threads to CPUs**:

  - Ensure threads access local memory by pinning them to specific CPUs.
  - Use OS-level APIs (e.g., `sched_setaffinity` on Linux).

- **Avoiding Thread Migration**:

  - Minimize context switching and migration to prevent remote memory access.

### **Memory Allocation Strategies**

- **Local Memory Allocation**:

  - Allocate memory on the same node as the executing thread.
  - Use NUMA-aware memory allocators (e.g., `libnuma` on Linux).

- **Data Placement Optimization**:

  - Analyze application data patterns to align frequently accessed data with local memory.

### **NUMA-Aware Data Structures**

- **Partitioned Data Models**:

  - Divide data across nodes to align with processing threads.
  - Example: Partition a trading book across NUMA nodes based on trading strategy.

- **Lock-Free Data Structures**:

  - Use atomic operations to reduce contention and overhead in multi-threaded environments.

### **NUMA-Friendly Scheduling**

- **OS Scheduler Configurations**:

  - Enable NUMA-aware scheduling to prioritize local CPU and memory usage.
  - Disable load balancing across NUMA nodes for critical trading threads.

- **Application-Level Scheduling**:

  - Implement custom thread pools aligned with NUMA topology.

---

## **Advanced Topics in NUMA for HFT**

### **Page Table Management**

- **Translation Lookaside Buffer (TLB)**:

  - Optimize page table usage to reduce TLB misses.
  - Use huge pages to reduce the number of required TLB entries.

- **Page Migration**:

  - Migrate frequently accessed pages to the node where they are most used.

### **RDMA and NUMA**

- **Remote Direct Memory Access (RDMA)**:

  - RDMA is a technique that enables direct memory access from one computer to another without involving the CPU.
  - RDMA is relevant in NUMA systems for high-throughput, low-latency networking. For example, RDMA is critical when transmitting trading data between NUMA nodes or different servers without the overhead of traditional TCP/IP stack processing.

- **NUMA-Aware Networking**:

  - Bind NICs (Network Interface Cards) to the NUMA node closest to the trading application. NICs are hardware components that connect a computer to a network, and in HFT, low-latency NICs are essential for fast order execution.

### **Cache Optimization**

- **Last Level Cache (LLC) Optimization**:

  - Align memory access patterns to maximize LLC utilization.
  - Use cache partitioning to isolate trading threads from other workloads.

- **Prefetching Techniques**:

  - Use hardware and software prefetching to reduce memory access latency.

---

## **Practical Implementation**

### **Step-by-Step NUMA Optimization**

1. **Analyze System Topology**:

   - Use tools like `lscpu`, `numactl`, or `hwloc` to map the NUMA layout.

2. **Align Data and Threads**:

   - Profile memory access patterns.
   - Bind threads and allocate memory accordingly.

3. **Tune Operating System Settings**:

   - Adjust scheduler policies.
   - Enable huge pages for memory management.

4. **Test and Validate**:

   - Measure performance using benchmarks like `perf` or custom tools.
   - Continuously refine data placement and thread affinity.

### **Case Study: NUMA Optimization for an HFT System**

- **Problem**: High cross-node memory traffic in an order matching engine.
- **Solution**:
  - Profile memory access patterns.
  - Bind threads handling matching logic to CPUs within the same node.
  - Use NUMA-aware data structures for order book storage.
- **Outcome**: Reduced latency by 20% and improved throughput.

### **Relevant Mock Data Examples**

- **Trading Book Partition**:

  - Node 0: Market data for stocks A-M.
  - Node 1: Market data for stocks N-Z.

- **Order Matching Engine**:

  - Node 0: Processes buy orders.
  - Node 1: Processes sell orders.

---

## **Computer Architecture Primer**

### **Processes vs. Threads**

- **Processes**: Independent execution units with their own memory space.
- **Threads**: Lighter-weight execution units sharing the same memory space within a process.

### **Memory Hierarchy**

1. **Registers**: Fastest but smallest storage (typically a few KB per core).
2. **Cache (L1, L2, L3)**: Larger but slower than registers (L1: 32 KB, L2: 256 KB, L3: 10-30 MB per CPU).
3. **RAM**: Larger and slower than cache (a few GB to several TB per node).
4. **Disk Storage**: Largest and slowest storage (several TB to PB in high-capacity systems).

### **NUMA Nodes**

- Each node in a NUMA system contains CPUs and memory.
- Interconnects link nodes for communication.

---

## **Conclusion**

Non-Uniform Memory Access (NUMA) is a cornerstone of high-performance computing in High-Frequency Trading (HFT). By understanding and leveraging NUMA's architecture, software design, and system configurations, HFT practitioners can achieve the low-latency, high-throughput performance required to stay competitive in the market.

Investing in NUMA expertise and continuous optimization is essential for maintaining cutting-edge HFT systems that capitalize on every microsecond of advantage.

