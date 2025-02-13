/*!
# Introduction

A lightweight async task executor library for bare metal (or any) systems.
 
This crate provides implementations for:
- Task management and unique task identification
- A simple FIFO-based executor for basic async operations
- A more sophisticated executor with proper waking mechanisms

The crate is `no_std` compatible and uses the `alloc` crate for heap allocations.

# Feature flags
* **x86_64** -
  When enabled, this will cause `exo_task` to use the x86_64 specific features.
  Currently, enabling this feature will allow `Executor::sleep_if_idle` to disable
  interrupts and halt the CPU when there are no tasks to process.
*/

extern crate alloc;

/// Core task types and traits for representing async computations
pub mod task;

/// Advanced executor implementation with proper waking support and task caching
pub mod executor;

/// Basic FIFO-based executor implementation for simple async operations
pub mod simple_executor;