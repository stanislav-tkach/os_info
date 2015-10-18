# os_type
Rust library to detect the operating system type

## Usage

Include this into your `Cargo.toml`:

```toml
[dependencies]
os_type="0.1.0"
```

In your code:

```rust
extern crate os_type;

fn foo() {
      match os_type::current_platform() {
        os_type::OSType::OSX => /*Do something here*/,
        _ => None
    }
}
```

Note that right now it detects only `OS X` and `RedHat/CentOS`. If you need support for more OS types,
I am looking forward to your Pull Request.

## License

MIT
