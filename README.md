[![Build Status](https://travis-ci.org/schultyy/os_type.svg?branch=master)](https://travis-ci.org/schultyy/os_type)

# os_type
Rust library to detect the operating system type

## Usage

Include this into your `Cargo.toml`:

```toml
[dependencies]
os_type="1.0.0"
```

In your code:

```rust
extern crate os_type;
let os = os_type::current_platform();
println!("Type: {:?}", os.os_type);
println!("Version: {}", os.version);
```

Right now, the following operating system types can be returned:
- Unknown
- Redhat
- CentOS
- OSX
- Ubuntu
- Debian
- Arch

If you need support for more OS types, I am looking forward to your Pull Request.

## Requirements

On Linux based systems this library requires that [lsb_release](http://refspecs.linuxbase.org/LSB_2.0.1/LSB-PDA/LSB-PDA/lsbrelease.html) is installed.

## Contributing

Bug reports and pull requests are welcome on [GitHub](https://github.com/schultyy/os_type).
You can find more information about contributing in the [CONTRIBUTING.md](https://github.com/schultyy/os_type/blob/master/CONTRIBUTING.md).
This project is intended to be a safe, welcoming space for collaboration and discussion, and contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org/version/1/4/) code of conduct.

## License

MIT
