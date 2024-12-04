# EnvMode

`EnvMode` is a simple Rust crate designed to help manage environment modes (development, production, staging) in your Rust applications. It provides utilities to check and validate the environment mode, making it easy to perform various actions based on the current environment mode.

## Features

- Supports 3 environment modes,
    - Development (`dev`)
    - Production (`prd`)
    - Staging (`stg`)

- Provides helper methods to check and validate environment modes.

## Installation

To use the `EnvMode` crate in your Rust application, add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
env_mode = "0.1.0"
serde = "1.0"
```

## Usage

Given below is an example of how to use the `EnvMode` crate in your application:

```rust
use serde::Deserialize;
use std::sync::Arc;
use env_mode::EnvMode;

fn main() {
    // Simulate getting an environment mode from a config or .env file
    let mode: Arc<str> = Arc::from("dev");

    // Convert the mode into the EnvMode enum
    let env_mode = EnvMode::from(&mode);

    // Check if it's the development mode
    if EnvMode::is_dev(&mode) {
        println!("Running in development mode.");
    }

    // Check if it's a valid mode
    if EnvMode::is_valid(&mode) {
        println!("The environment mode is valid.");
    }
}
```

### Methods

`EnvMode::is_dev(mode: &str) -> bool`

Checks if the provided mode is the development mode.

```rust
assert_eq!(EnvMode::is_dev("dev"), true);
```

`EnvMode::is_prd(mode: &str) -> bool`

Checks if the provided mode is the production mode.

```rust
assert_eq!(EnvMode::is_prd("prd"), true);
```

`EnvMode::is_stg(mode: &str) -> bool`

Cheks if the provided mode is the staging mode.

```rust
assert_eq!(EnvMode::is_stg("stg"), true);
```

`EnvMode::is_valid(mode: &str) -> bool`

Checks if the mode is one of the valid environment modes (`dev`, `prd`, `stg`).

```rust
assert_eq!(EnvMode::is_valid("dev"), true);
assert_eq!(EnvMode::is_valid("test"), false); // Invalid mode
```

### Enum variants

- `EnvMode::Dev` - Development mode
- `EnvMode::Prd` - Production mode
- `EnvMode::Stg` - Staging mode
