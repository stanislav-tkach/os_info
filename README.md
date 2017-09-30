# os_info

[![Build Status](https://travis-ci.org/DarkEld3r/os_info.svg?branch=master)](https://travis-ci.org/DarkEld3r/os_info)
[![Build status](https://ci.appveyor.com/api/projects/status/43s26ds9r8t5xwnu?svg=true)](https://ci.appveyor.com/project/DarkEld3r/os-info)
[![Latest Version](http://meritbadge.herokuapp.com/os_info)](https://crates.io/crates/os_info)
[![License](https://img.shields.io/github/license/darkeld3r/os_info.svg)](https://github.com/darkeld3r/os_info)

## Overview

Library for detecting the operating system type and version.

Based on [os_type](https://github.com/schultyy/os_type) by Jan Schulte. The main difference of `os_info` is that this library separates completely incompatible operating systems by conditional compilation.

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
