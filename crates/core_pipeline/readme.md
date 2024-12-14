# core_pipeline Crate

## Overview

The `core_pipeline` crate implements the critical real-time data processing path of the high-frequency trading system. This is where raw, normalized market data from the `reception_layer` is transformed into actionable trading signals.

The crate encompasses:

1. **Order Book Management:**  
   Maintains a high-performance in-memory limit order book. This is central to understanding the current market state and the best prices at which trades can occur.

2. **Matching Engine:**  
   Matches incoming orders against the order book, producing trades and updates in real-time.

3. **Risk Checks:**  
   Ensures that incoming orders comply with risk constraints, preventing catastrophic losses or unintended positions.

4. **Aggregator:**  
   Computes short-term metrics (like EMA) and can integrate simple regression or other advanced analytics to provide immediate, low-latency signals (e.g., detecting price momentum).

5. **Signal Generator:**  
   Translates aggregated metrics and market conditions into trading signals that can be sent to an execution system or strategy component.

6. **Support Utilities (Allocators, Lock-Free Queues, Timing):**  
   Provides low-level infrastructure to keep latency as low as possible (e.g., custom allocators, lock-free communication structures, and high-resolution timing utilities).

## Design Goals

- **Ultra-Low Latency:**  
  Every microsecond counts. Data structures and functions are chosen and tuned for minimal overhead.

- **Deterministic Performance:**  
  Where possible, minimize unpredictable operations (like memory allocations) in the hot path.

- **Modularity:**  
  Each component (order book, matching engine, aggregator) is split into its own module, making it easier to test and optimize independently.

## Example Flow

1. **Incoming Data:** A new quote arrives from the `reception_layer`.
2. **Order Book Update:** The order book updates its state with the new price level.
3. **Aggregator:** The aggregator recalculates the EMA with the latest prices.
4. **Signal Generator:** If the EMA crosses a certain threshold, generate a BUY signal.
5. **Risk Checks:** Validate that the BUY signal won't violate limits.

These steps occur repeatedly at high frequency.

---

## Status

This crate is central to the system and will be continuously tuned. Over time, more sophisticated aggregations (e.g., regressions) and advanced order types may be added. Benchmarks and profiling tools should be run regularly to maintain latency targets.
