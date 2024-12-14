
# circular_shared

A high-performance, low-latency circular buffer designed for one producer and multiple consumers. The buffer is backed by `mmap` on `/dev/shm/` and can optionally be pinned in RAM (`mlock`) and use huge pages for even lower latency.

sudo apt-get install musl-tools gcc-multilib make


---

## Features

- **One Producer, Multiple Consumers**: 
  - The producer writes data at a high rate, incrementing a shared `head` pointer.
  - Multiple consumers can attach to the same buffer and read the newest data. Each consumer manages its own read index, allowing independent consumption rates.
  
- **Low Latency & High Throughput**:
  - Memory-mapped shared memory (`/dev/shm`) for near-RAM-speed access.
  - Optional `mlock` to keep data in RAM and avoid paging.
  - Optional huge page usage for reduced TLB misses.
  - Minimal locking. The producer updates `head` atomically, and consumers perform lock-free reads if desired.
  
- **Configurable**:
  - Set the number of rows and columns.
  - Assign column names and types (currently example assumes `f64` for simplicity, but the schema is recorded in JSON for future extensions).
  - Choose whether to lock reads/writes.
  - Control memory pinning and huge pages via builder options.

- **Easy Integration**:
  - The crate prints out the shared memory file path (e.g., `/dev/shm/cbuf_12345_67890`), so other processes or tools can attach.
  - Consumers can map the same file and use the provided methods to read rows.
  - For visualization, use Python/NumPy: 
    ```python
    import numpy as np
    data = np.fromfile("/dev/shm/cbuf_12345_67890", dtype='float64')
    data = data.reshape(num_rows, num_cols)
    ```

---

## How It Works

### Initialization

1. **Producer**:
   - The producer creates the buffer using `CircularBufferBuilder`.
   - The configuration (columns, sizes, etc.) is written into the shared memory header in JSON.
   - The data area follows the header.

2. **Consumers**:
   - Consumers attach to the buffer by specifying the same file path or by being given the path by the producer.
   - They map the memory region and use the exposed `head` value to calculate the valid range of rows to read.

---

### Writing Data
1. The producer calls `write()` with a slice of `f64` values.
2. The buffer writes the row at `head % capacity`.
3. `head` is incremented atomically.
4. If the buffer is full, the oldest data is overwritten.

---

### Reading Data

To read from the circular buffer:
1. **Extract the `head` Value**:
   - The `head` tracks the next write position and determines the valid data range.
   - Consumers can request the `head` from shared memory, e.g., using the crate’s API or directly mapping the memory.

2. **Determine Valid Range**:
   - If `head < capacity`, valid rows are in the range `[0, head)`.
   - If `head >= capacity`, valid rows are in the range `[head - capacity, head)`.
   - Use modulo arithmetic to wrap indices: `index % capacity`.

3. **Access Data**:
   - Map the shared memory region in Python using `mmap` or `numpy.fromfile`.
   - Slice the buffer based on the calculated valid range.

---

## Python Integration

The producer exposes the shared memory file and the current `head` position. Consumers can read this data using Python.

### Requesting the `head` Value
In Python, use the `mmap` module to access the `head` value stored in the shared memory header:

```python
import mmap
import struct

def read_head(file_path):
    """
    Reads the head pointer from the shared memory header.

    :param file_path: Path to the shared memory file (e.g., '/dev/shm/cbuf_12345_67890').
    :return: The current value of the head pointer (integer).
    """
    with open(file_path, "r+b") as f:
        mm = mmap.mmap(f.fileno(), 0, access=mmap.ACCESS_READ)
        # The head is stored as an atomic usize after the SharedHeader
        head_offset = struct.calcsize("Q")  # Assuming SharedHeader starts with a usize
        head = struct.unpack_from("Q", mm, head_offset)[0]
        mm.close()
        return head
```

### Reading Data with NumPy
Use the `head` value to determine the valid rows in the buffer:

```python
import numpy as np

def read_circular_buffer(file_path, num_rows, num_cols, head):
    """
    Reads valid data from a circular buffer in shared memory.

    :param file_path: Path to the shared memory file.
    :param num_rows: Total number of rows in the buffer.
    :param num_cols: Number of columns in the buffer.
    :param head: The current head position (read using read_head).
    :return: A NumPy array of valid rows.
    """
    # Map the buffer
    data = np.memmap(file_path, dtype='float64', mode='r')
    data = data.reshape(num_rows, num_cols)
    
    # Determine the valid range
    if head <= num_rows:
        # No wraparound yet
        valid_data = data[:head]
    else:
        # Wraparound occurred
        start = head % num_rows
        valid_data = np.vstack((data[start:], data[:start]))
    
    return valid_data
```

### Complete Example
```python
# Example shared memory file
file_path = "/dev/shm/cbuf_12345_67890"

# Metadata (should be retrieved from the producer or configuration)
num_rows = 100  # Capacity of the buffer
num_cols = 2    # Number of columns (e.g., "time" and "value")

# Read the head pointer
head = read_head(file_path)
print(f"Current head: {head}")

# Read valid rows from the circular buffer
valid_data = read_circular_buffer(file_path, num_rows, num_cols, head)

# Print the data
print("Valid rows in the buffer:")
print(valid_data)
```

---

## Notes and Advices

1. **OS Modifications for Lower Latency**:
   - **Ensure `/dev/shm` is RAM-backed**: Verify that `/dev/shm` is configured as a tmpfs filesystem in RAM, not a swap-backed system.
   - **Huge Pages**:
     - Configure your system for [Huge Pages](/assets/huge_page.md) (`/proc/sys/vm/nr_hugepages`) to reduce TLB misses. Use the `use_hugepages(true)` option in the builder to enable this feature.
     - Use `madvise` with `MADV_HUGEPAGE` or `MADV_NOHUGEPAGE` to fine-tune huge page usage.
   - **Real-Time Scheduling**:
     - For ultra-low latency, elevate the process priority using `sched_setscheduler` or tools like `chrt`. You can set the thread priority directly via Rust's FFI.
   - **CPU Pinning**:
     - Use `taskset` or `pthread_setaffinity_np` in Rust to pin producer and consumer threads to specific cores. This reduces context-switch overhead.

2. **Handling Multiple Consumers**:
   - Each consumer tracks its own read index. The system does not enforce a global `tail` pointer.
   - Data older than `head - capacity` is overwritten. Ensure consumers read fast enough to avoid missing data.

3. **Atomic Snapshots**:
   - To capture atomic snapshots, consider implementing versioned headers or a double-buffering strategy. This prevents partial reads during write operations in high-concurrency scenarios.

---

## Improvements

- **Multiple Data Types**: Currently, all columns are `f64`. Extend to multiple data types by storing type info in the header and handling varying sizes.
- **Atomic Snapshots**: If required, implement a double-buffering strategy or versioning for atomic snapshots.
- **Huge Pages**: Ensure the system is configured with huge pages, and consider using `madvise` to hint the kernel.
- **Real-Time Scheduling**: Use OS-level tools or `sched_setscheduler` from Rust's FFI to achieve even lower latency.

---

This updated README provides a clear and thorough explanation of how to interact with the circular buffer, including handling the `head` position and reading the buffer using Python.