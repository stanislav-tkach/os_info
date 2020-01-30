# os_info

<!-- cspell:disable -->
**Status:**
[![Travis Build Status](https://img.shields.io/travis/DarkEld3r/os_info/master.svg?label=Linux)](https://travis-ci.org/DarkEld3r/os_info)
[![CircleCI Build Status](https://img.shields.io/circleci/project/github/DarkEld3r/os_info/master.svg?label=MacOS)](https://circleci.com/gh/darkeld3r/os_info/tree/master)
[![AppVeyor Build Status](https://img.shields.io/appveyor/ci/darkeld3r/os-info/master.svg?label=Windows)](https://ci.appveyor.com/project/DarkEld3r/os-info/branch/master)
<!-- cspell:enable -->

**Project info:**
[![Docs.rs](https://docs.rs/os_info/badge.svg)](https://docs.rs/os_info)
[![Latest Version](http://meritbadge.herokuapp.com/os_info)](https://crates.io/crates/os_info)
[![License](https://img.shields.io/github/license/darkeld3r/os_info.svg)](https://github.com/darkeld3r/os_info)

**Project details:**
[![LoC](https://tokei.rs/b1/github/darkeld3r/os_info)](https://github.com/darkeld3r/os_info)
![rust 1.32+ required](https://img.shields.io/badge/rust-1.32+-blue.svg?label=Required%20Rust)
[![dependency status](https://deps.rs/repo/github/darkeld3r/os_info/status.svg)](https://deps.rs/repo/github/darkeld3r/os_info)

## Overview

Library for detecting the operating system type and version.

Based on [os_type](https://github.com/schultyy/os_type). The main difference of
`os_info` is that this library separates completely incompatible operating
systems by conditional compilation and uses specific system API whenever is
possible.

## Usage

To use this crate, add `os_info` as a dependency to your project's Cargo.toml:

```toml
[dependencies]
os_info = "1.3.3"
```

## Example

```rust
let info = os_info::get();

// Print full information:
println!("OS information: {}", info);

// Print information separately:
println!("Type: {}", info.os_type());
println!("Version: {}", info.version());
println!("Bitness: {}", info.bitness());
```

Right now, the following operating system types can be returned:

- Unknown
- Android
- Emscripten
- Linux
- Redhat
- RedHatEnterprise
- Ubuntu
- Debian
- Arch
- CentOS
- Fedora
- Amazon
- Alpine
- Macos
- Redox
- Windows

If you need support for more OS types, I am looking forward to your Pull
Request.

## Requirements

On Linux based systems this library requires that [lsb_release] is installed.

## License

`os_info` is licensed under the MIT license. See [LICENSE] for the details.

[lsb_release]: http://refspecs.linuxbase.org/LSB_2.0.1/LSB-PDA/LSB-PDA/lsbrelease.html
[LICENSE]: https://github.com/darkeld3r/os_info/blob/master/LICENSE
