# exo_task

A lightweight async task executor for bare metal (or any) systems, supporting `no_std` environments.

## Features

- Fully `no_std` compatible (requires `alloc`)
- Two executor implementations:
  - Simple FIFO-based executor for basic needs
  - Advanced executor with proper waking mechanisms
- Configurable platform support (x86_64 feature flag)
- Efficient task management with unique task IDs
- Waker caching for better performance
- Simple event system

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
exo_task = "0.1.2"
```

For x86_64 platform support without `std`:

```toml
[dependencies]
exo_task = { version = "0.1.2", default-features = false, features = ["x86_64"] }
```

## Example

```rust
use exo_task::{Task, SimpleExecutor};

async fn example_task() {
    // Your async code here
}

let mut executor = SimpleExecutor::new();
executor.spawn(Task::new(example_task()));
executor.run();
```

```rust
use crate::{EventBus, EventListener};
use alloc::string::String;

// Define some example event types
#[derive(Debug)]
struct TaskCreatedEvent {
    task_id: u32,
    name: String,
}

#[derive(Debug)]
struct TaskCompletedEvent {
    task_id: u32,
}

// Example usage
pub fn event_system_example() {
    // Create an event bus
    let mut event_bus = EventBus::new();

    // Register listeners for different event types
    let task_created_listener = EventListener::new(|event: &TaskCreatedEvent| {
        println!("Task created: #{} - {}", event.task_id, event.name);
    });

    let task_completed_listener = EventListener::new(|event: &TaskCompletedEvent| {
        println!("Task completed: #{}", event.task_id);
    });

    // Another listener for the same event type
    let task_created_logger = EventListener::new(|event: &TaskCreatedEvent| {
        println!("[LOG] New task created with ID: {}", event.task_id);
    });

    // Subscribe listeners to the event bus
    event_bus.subscribe(task_created_listener);
    event_bus.subscribe(task_created_logger);
    event_bus.subscribe(task_completed_listener);

    // Emit some events
    event_bus.emit(TaskCreatedEvent {
        task_id: 1,
        name: String::from("Initialize system"),
    });

    event_bus.emit(TaskCompletedEvent { task_id: 1 });
}
```

## Requirements

- Rust nightly
- Working allocator implementation

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Note: This licensing applies only to the current branch.

### Credits

This crate is based on [Philipp Oppermann's Async/Await](https://os.phil-opp.com/async-await/) implementation from his "Writing an OS in Rust" series, adapted into a standalone crate for broader use.

Documentation and comments were enhanced using GitHub Copilot while preserving the original code functionality.

### Contributing

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Status

This project is currently in development. API may change between versions.