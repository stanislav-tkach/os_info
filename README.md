# os_info

[![Build Status](https://travis-ci.org/DarkEld3r/os_info.svg?branch=master)](https://travis-ci.org/DarkEld3r/os_info)
[![Build status](https://ci.appveyor.com/api/projects/status/7ccw7aupq33we07r?svg=true)](https://ci.appveyor.com/project/DarkEld3r/os-version)

## Overview

Library for detecting the operating system type and version.

Based on [os_type](https://github.com/schultyy/os_type) by Jan Schulte.

## Usage

To use this crate, add `os_info` as a dependency to your project's Cargo.toml:

```toml
[dependencies]
os_info = "0.1.0"
```

## Example

```rust
extern crate os_info;

let os = os_info::current_platform();
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

## License

`os_info` is licensed under the MIT license. See [LICENSE](https://github.com/darkeld3r/os_info/blob/master/LICENSE) for the details.
