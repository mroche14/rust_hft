# High-Frequency Trading (HFT) Scheduling Policies: Key Insights

## **1. Scheduling Policies Overview**

### **SCHED_NORMAL (Completely Fair Scheduler - CFS)**
- **Purpose**: Default time-sharing scheduler for general-purpose tasks.
- **Behavior**:
  - Allocates CPU time based on fairness and priority.
  - Preemptive scheduling ensures responsiveness for interactive tasks.
- **Usage**: Non-real-time tasks in HFT systems, such as background data processing.

### **SCHED_FIFO (First-In-First-Out)**
- **Purpose**: Real-time scheduling for latency-critical tasks.
- **Behavior**:
  - Tasks run until they yield, block, or are preempted by a higher-priority task.
  - No time slicing.
- **Usage**: Critical HFT tasks, such as mock data generation or order placement.
- **Key Considerations**:
  - Requires careful design to avoid priority inversion.
  - Ensures deterministic task execution.

### **SCHED_RR (Round-Robin)**
- **Purpose**: Real-time scheduling with fairness among tasks of the same priority.
- **Behavior**:
  - Similar to `SCHED_FIFO` but introduces time slices for tasks at the same priority.
  - Prevents monopolization of the CPU by a single task.
- **Usage**: Suitable for concurrent trading algorithms requiring fair execution.

### **SCHED_BATCH**
- **Purpose**: Optimized for long-running, CPU-bound batch jobs.
- **Behavior**:
  - Runs at a lower priority than `SCHED_NORMAL` tasks.
  - Utilizes idle CPU cycles.
- **Usage**: Background data processing in HFT systems.

### **SCHED_IDLE**
- **Purpose**: For tasks with the lowest priority, utilizing the CPU only when idle.
- **Usage**: Non-critical background maintenance tasks.

### **SCHED_DEADLINE**
- **Purpose**: For hard real-time tasks with specific deadlines.
- **Behavior**:
  - Tasks declare execution time and deadlines.
  - Scheduler ensures deadlines are met based on CPU capacity.
- **Usage**: Real-time control systems or time-sensitive trading strategies.

---

## **2. Real-Time Scheduling in HFT Systems**

### **Key Attributes**
1. **Determinism**:
   - Ensures tasks execute within predictable timing constraints.
   - Critical for processing market data and placing orders in HFT.

2. **Preemption**:
   - Higher-priority tasks preempt lower-priority ones.
   - Essential for ensuring critical tasks like order placement are not delayed.

3. **Latency**:
   - Minimized by prioritizing critical tasks and reducing context-switch overhead.

4. **Avoiding Starvation**:
   - Use techniques like priority inheritance to handle priority inversion scenarios.

---

## **3. NUMA and CPU Affinity**

### **NUMA (Non-Uniform Memory Access)**
- In a NUMA architecture, memory access times depend on the processor accessing the memory.
- **NUMA Support in HFT**:
  - Binds threads to specific NUMA nodes to reduce latency and improve performance.
  - Ensures data locality by aligning thread execution with memory locations.

### **CPU Affinity**
- **Definition**: Binds threads to specific CPU cores to reduce scheduling overhead and improve cache efficiency.
- **Benefits**:
  - Reduces context-switch overhead.
  - Ensures predictable performance for latency-sensitive tasks.

### **Relation Between NUMA and CPU Affinity**
- Both techniques work together:
  - NUMA optimizes memory access by localizing memory to a specific processor.
  - CPU affinity ensures tasks utilize the optimal cores associated with the NUMA node.

---

## **4. Lock-Free Circular Buffers in HFT**

### **Challenges**:
- Data races and contention when multiple readers and writers access the buffer concurrently.

### **Solutions**:
1. **Atomic Operations**:
   - Use `fetch_add` to atomically update the head pointer, ensuring safe writes.

2. **Reader-Writer Index Separation**:
   - Separate read and write pointers to avoid contention.

3. **Lock-Free Reads**:
   - Readers access data without blocking, improving throughput and latency.

4. **Memory Ordering**:
   - Use `Acquire`/`Release` semantics to ensure visibility of writes to readers.

### **Benefits for HFT**:
- Lock-free reads ensure consumers process market data without delay.
- High throughput and low latency are maintained under heavy load.

---

## **5. Key Takeaways for HFT Systems**

- **Scheduling Policies**: Use real-time policies like `SCHED_FIFO` or `SCHED_RR` for deterministic task execution.
- **NUMA and CPU Affinity**: Optimize thread and memory locality to reduce latency.
- **Lock-Free Data Structures**: Implement lock-free circular buffers for high-performance data sharing.
- **Real-Time Optimizations**:
  - Apply OS-level tuning (e.g., memory locking, huge pages) to ensure predictable performance.

By combining these techniques, HFT systems achieve the ultra-low-latency and high-throughput performance necessary for competitive trading.