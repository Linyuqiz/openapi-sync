# openapi-sync

[![Crates.io](https://img.shields.io/crates/v/openapi-sync.svg)](https://crates.io/crates/openapi-sync)
[![Documentation](https://docs.rs/openapi-sync/badge.svg)](https://docs.rs/openapi-sync)
[![License](https://img.shields.io/crates/l/openapi-sync.svg)](LICENSE)

A Rust implementation of a file synchronization tool for company SDK resources.

## Features

- **Efficient File Synchronization**: Smart detection of changes to minimize file operations
- **Task Management**: Flexible API for defining and managing synchronization tasks
- **Customizable Sync Rules**: Define which files to sync and how they should be processed
- **Robust Error Handling**: Comprehensive error reporting and recovery mechanisms
- **Async Support**: Built on tokio for efficient asynchronous file operations
- **Minimal Configuration**: Simple setup with sensible defaults

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openapi-sync = "0.1.0"
```

## Quick Start

```rust
use openapi_sync::syncer::Syncer;
use openapi_sync::task::Task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new syncer instance
    let syncer = Syncer::new();
    
    // Create and configure a synchronization task
    let task = Task::new();
    
    // Add the task to the syncer
    syncer.add_task(task);
    
    // Start the synchronization process
    syncer.start();
    
    Ok(())
}
```

## Documentation

For more detailed documentation, please visit [docs.rs/openapi-sync](https://docs.rs/openapi-sync).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.