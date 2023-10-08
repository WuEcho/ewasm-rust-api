# ewasm-rust-api

# Usage

Add the dependency, as usual:
```toml
[dependencies]
ewasm_api = { git = "https://github.com/WuEcho/ewasm-rust-api", tag = "0.9" }
```

In your project, include the prelude:
```rust
use ewasm_api::prelude::*;
```

Other modules are available as well, outside of the prelude. Refer to the documentation for more info.

`ewasm-rust-api` builds with various feature sets:
- `default`: Builds with `wee_alloc` as the global allocator and with the Rust standard library.
- `qimalloc`: Builds with [qimalloc](https://github.com/wasmx/qimalloc) as the global allocator.
- `debug`: Exposes the debugging interface.
- `experimental`: Exposes the experimental bignum system library API.

Further documentation is available [here](https://docs.rs/ewasm_api/).

## Author(s)

Alex Beregszaszi, Jake Lang

## License

Apache 2.0


