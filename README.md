[![Build Status](https://travis-ci.org/schultyy/os_type.svg?branch=master)](https://travis-ci.org/schultyy/os_type)

# os_type
Rust library to detect the operating system type

## Usage

Include this into your `Cargo.toml`:

```toml
[dependencies]
os_type="0.3.0"
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

Right now, the following operating systems are detected:

- Mac OS X
- CentOS
- RedHat
- Ubuntu

If you need support for more OS types, I am looking forward to your Pull Request.

## License

MIT
