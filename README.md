Deferred Box
============

Defer the value set after the struct has been initialized.

## Installation

Add these lines to `Cargo.toml` under the `[dependencies]` section:

```toml
deferred-box = "0.1"
```

## Usage

```rust
let deferred_box = DeferredBox::new();
deferred_box.get(); // None
deferred_box.set(42);
deferred_box.get(); // Some(&42)
```

## License

MIT License