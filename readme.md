## recoverable-spawn

[![](https://img.shields.io/crates/v/recoverable-spawn.svg)](https://crates.io/crates/recoverable-spawn)
[![](https://docs.rs/recoverable-spawn/badge.svg)](https://docs.rs/recoverable-spawn)
[![](https://img.shields.io/crates/l/recoverable-spawn.svg)](./LICENSE)
[![](https://github.com/ltpp-universe/recoverable-spawn/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/recoverable-spawn/actions?query=workflow:Rust)

[Official Documentation](https://docs.ltpp.vip/recoverable-spawn/)

[Api Docs](https://docs.rs/recoverable-spawn/latest/recoverable_spawn/)

> A library to automatically restart a thread after a panic. This is useful for ensuring that threads continue running even in the event of an unexpected error. It provides a simple and effective mechanism to catch panics, restart the thread, and optionally log the error for monitoring and debugging purposes.

## Installation

To use this crate, you can run cmd:

```shell
cargo add recoverable-spawn
```

## Use

### recoverable_spawn

```rust
use recoverable_spawn::*;
let msg: &str = "test";
let handle: JoinHandle<()> = recoverable_spawn(move || {
    panic!("{}", msg);
});
let _ = handle.join();
```

### recoverable_spawn_with_error_handle

```rust
use recoverable_spawn::*;
let msg: &str = "test";
let handle: JoinHandle<()> = recoverable_spawn_with_error_handle(
    move || {
        panic!("{}", msg);
    },
    |err| {
        println!("handle error => {}", err);
    },
);
let _ = handle.join();
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
