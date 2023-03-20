# Change Log

All notable changes to this project will be documented in this file.

## [Unreleased]

## [3.7.0] (2023-03-20)

- Information about a processor's architecture has been added. (#336)

- Mabox Linux support has been added. (#338)

- Alpaquita Linux support has been added. (#340)

- Artix Linux support has been added. (#342)

## [3.6.0] (2023-01-30)

- OpenCloudOS support has been added. (#328)

- openEuler support has been added. (#328)

- Arch Linux ARM and Debian ARM detection has been improved. (#331)

## [3.5.1] (2022-09-19)

- Windows 11 detection has been fixed. (#322)

## [3.5.0] (2022-08-01)

- Red Hat Enterprise Linux detection has been improved. (#311)

- Garuda Linux support has been added. (#314)

- The operating system detection from the release files has been improved. (#317)

## [3.4.0] (2022-05-22)

- Gentoo Linux support has been added. (#307)

- FreeSBD detection has been fixed. (#309)

## [3.3.0] (2022-05-01)

- Fedora 35 (without `lsb_release`) detection has been fixed. (#293)

- HardenedBSD support has been added. (#295)

- Mariner support has been added. (#299)

- Illumos support has been added. (#305)

## [3.2.0] (2022-02-04)

- MidnightBSD support has been added. (#290)

- Bitness detection has been fixed for SPARC v9 platform on NetBSD and OpenBSD. (#291)

## [3.1.0] (2022-01-14)

- OpenBSD support has been added. (#286)

## [3.0.9] (2021-12-21)

- NetBSD bitness detection has been fixed. (#283)

## [3.0.8] (2021-11-23)

- NetBSD support has been added. (#279)

- Fedora 35 detection has been fixed. (#281)

## [3.0.7] (2021-08-12)

- CentOS Stream detection has been fixed. (#267)

- `env_logger` version has been updated to `0.9.0`. (#269)

## [3.0.6] (2021-05-17)

- Compilation for the DragonFly BSD operating system has been fixed. (#264)

## [3.0.5] (2021-05-13)

- Raspberry Pi OS support has been added. (#262)

## [3.0.4] (2021-04-28)

- Compilation for the DragonFly BSD operating system has been fixed. (#260)

## [3.0.3] (2021-04-26)

- DragonFly BSD support has been added. (#256)

- Compilation for the FreeBSD operating system has been fixed. (#258)

## [3.0.2] (2021-04-05)

- NixOS support has been added. (#252)

- FreeBSD support has been added. (#253)

## [3.0.1] (2020-10-23)

- Linux Mint support has been added. (#233)

- Operating system type detection has been fixed for the following Linux distributions:
  - Amazon Linux. (#225, #226)
  - Red Hat Enterprise Linux. (#228)
  - Suse. (#229)
  - Fedora. (#230)
  - Alpine Linux. (#235)

- Log level for the message about the absent `lsb_release` has been reduced
  from `warn` to `debug`. (#237)

## [3.0.0] (2020-09-28)

- `VersionType` enum has been extended to support the rolling release
  versioning. (#211)

- The codename field has been added to `Version`. (#213, #214)

- The `Display` trait implementation for `Version`, `Info` and `Bitness` has
  been changed. (#219)

- The edition filed has been moved from `Version` to `Info`. (#219)

- `VersionType` has been renamed to `Version`. (#219)

## [2.0.8] (2020-08-10)

- EndeavourOS support has been added. (#210)

## [2.0.7] (2020-07-26)

- Arch Linux detection from the release file has been fixed. (#206)

## [2.0.6] (2020-05-24)

- Pop!_OS support has been added. (#173)

- Manjaro support has been added. (#180)

- Solus support has been added. (#182)

## [2.0.5] (2020-05-13)

- The markdown (README.md) test has been fixed. (#167)

## [2.0.4] (2020-05-11)

- The issue with `README.md` and `LICENSE` files not being included into the package
  has been fixed. (#166)

## [2.0.3] (2020-05-10)

- Centos detection has been fixed. (#162)

- Executable has been moved to a separate crate to reduce library dependencies. (#163)

## [2.0.2] (2020-03-15)

- Oracle Linux support has been added. (#153)

## [2.0.1] (2020-02-22)

- Bitness detection has been implemented for MacOS. (#147)

- `regex` dependency has been removed. (#144)

- `libntdll.a` has been removed from the sources. (#146)

## [2.0.0] (2020-02-11)

- `Bitness` and `Type` enums have been marked as `non_exhaustive`. (#140)

- SUSE Enterprise and openSUSE support has been added. (#135)

- `serde` has been made an optional dependency. (#138)

## [1.3.3] (2020-01-24)

- `Version`'s `Display` implementation has been fixed. (#128)

## [1.3.2] (2020-01-19)

- Bitness detection has been implemented for Linux. (#125)

## [1.3.1] (2020-01-15)

- Missing winapi features have been added. (#123)

## [1.3.0] (2020-01-14)

- `Info` has been extended with operating system bitness. See `Info::bitness`
  for details. Currently implemented only for Windows. (#119)

- `Info`'s `Display` implementation has been fixed. (#113)

- `winapi` dependency version has been updated to `0.3`. (#115)

## [1.2.0] (2019-11-11)

- Distinguish between Red Hat and Red Hat Enterprise operating system versions. (#107)

## [1.1.3] (2019-09-11)

- Amazon Linux support has been added. (#105)

## [1.1.2] (2019-09-11)

- OS X beta versions support has been added. (#103)

## [1.1.1] (2019-03-22)

- Migrate to the 2018 edition. (#96)

- Fix deprecation warnings. (#95)

- Update dependencies versions. (#94)

## [1.1.0] (2018-10-13)

- Serialization support (`serde`) has been added to all public data types. (#91)

- Dependencies have been updated. (#92, #93)

## [1.0.3] (2018-09-23)

- Fixed linker errors for 32-bit msvc builds. (#88)  

## [1.0.2] (2018-09-05)

- Fixed issue with incorrect Linux version detection. (#85).

## [1.0.1] (2018-05-27)

- Compilation has been fixed for all "unsupported" operating systems. Now
  `Unknown` `Type` is returned in such cases.

## [1.0.0] (2018-05-21)

- Better Linux, Windows and OS X support.

- Numerous code improvements.

- More and better testing.

- Logging.

## [0.7.0] (2018-01-21)

- Fixed issued with OS X build.

## [0.6.0] (2017-12-17)

- Alpine Linux support (#48).

- Use release files if possible (#48).

## [0.5.0] (2017-12-13)

- Windows support (#45).

- Fedora support (#46).

## [0.4.0] (2017-11-12)

- Redox support (untested).

## [0.3.0] (2017-11-02)

- Fix OS X build.

## [0.2.0] (2017-11-02)

- Add "dead" links check (#32).

- Fix Travis badge on crates.io.

- Travis OS X build (#35).

## [0.1.1] (2017-10-03)

- Examples have been updated slightly.

- Readme has been updated.

## [0.1.0] (2017-10-03)

The first release containing only minor infrastructural changes and based on [os_type](https://github.com/schultyy/os_type).

[Unreleased]: https://github.com/stanislav-tkach/os_info/compare/v3.7.0...HEAD
[3.7.0]: https://github.com/stanislav-tkach/os_info/compare/v3.6.0...v3.7.0
[3.6.0]: https://github.com/stanislav-tkach/os_info/compare/v3.5.1...v3.6.0
[3.5.1]: https://github.com/stanislav-tkach/os_info/compare/v3.5.0...v3.5.1
[3.5.0]: https://github.com/stanislav-tkach/os_info/compare/v3.4.0...v3.5.0
[3.4.0]: https://github.com/stanislav-tkach/os_info/compare/v3.3.0...v3.4.0
[3.3.0]: https://github.com/stanislav-tkach/os_info/compare/v3.2.0...v3.3.0
[3.2.0]: https://github.com/stanislav-tkach/os_info/compare/v3.1.0...v3.2.0
[3.1.0]: https://github.com/stanislav-tkach/os_info/compare/v3.0.9...v3.1.0
[3.0.9]: https://github.com/stanislav-tkach/os_info/compare/v3.0.8...v3.0.9
[3.0.8]: https://github.com/stanislav-tkach/os_info/compare/v3.0.7...v3.0.8
[3.0.7]: https://github.com/stanislav-tkach/os_info/compare/v3.0.6...v3.0.7
[3.0.6]: https://github.com/stanislav-tkach/os_info/compare/v3.0.5...v3.0.6
[3.0.5]: https://github.com/stanislav-tkach/os_info/compare/v3.0.4...v3.0.5
[3.0.4]: https://github.com/stanislav-tkach/os_info/compare/v3.0.3...v3.0.4
[3.0.3]: https://github.com/stanislav-tkach/os_info/compare/v3.0.2...v3.0.3
[3.0.2]: https://github.com/stanislav-tkach/os_info/compare/v3.0.1...v3.0.2
[3.0.1]: https://github.com/stanislav-tkach/os_info/compare/v3.0...v3.0.1
[3.0.0]: https://github.com/stanislav-tkach/os_info/compare/v2.0.8...v3.0
[2.0.8]: https://github.com/stanislav-tkach/os_info/compare/v2.0.7...v2.0.8
[2.0.7]: https://github.com/stanislav-tkach/os_info/compare/v2.0.6...v2.0.7
[2.0.6]: https://github.com/stanislav-tkach/os_info/compare/2.0.5...v2.0.6
[2.0.5]: https://github.com/stanislav-tkach/os_info/compare/2.0.4...2.0.5
[2.0.4]: https://github.com/stanislav-tkach/os_info/compare/v2.0.3...2.0.4
[2.0.3]: https://github.com/stanislav-tkach/os_info/compare/v2.0.2...v2.0.3
[2.0.2]: https://github.com/stanislav-tkach/os_info/compare/v2.0.1...v2.0.2
[2.0.1]: https://github.com/stanislav-tkach/os_info/compare/v2.0...v2.0.1
[2.0.0]: https://github.com/stanislav-tkach/os_info/compare/v1.3.3...v2.0
[1.3.3]: https://github.com/stanislav-tkach/os_info/compare/v1.3.2...v1.3.3
[1.3.2]: https://github.com/stanislav-tkach/os_info/compare/v1.3.1...v1.3.2
[1.3.1]: https://github.com/stanislav-tkach/os_info/compare/v1.3...v1.3.1
[1.3.0]: https://github.com/stanislav-tkach/os_info/compare/v1.2...v1.3
[1.2.0]: https://github.com/stanislav-tkach/os_info/compare/v1.1.3...v1.2
[1.1.3]: https://github.com/stanislav-tkach/os_info/compare/v1.1.2...v1.1.3
[1.1.2]: https://github.com/stanislav-tkach/os_info/compare/v1.1.1...v1.1.2
[1.1.1]: https://github.com/stanislav-tkach/os_info/compare/v1.1...v1.1.1
[1.1.0]: https://github.com/stanislav-tkach/os_info/compare/v1.0.3...v1.1
[1.0.3]: https://github.com/stanislav-tkach/os_info/compare/v1.0.2...v1.0.3
[1.0.2]: https://github.com/stanislav-tkach/os_info/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/stanislav-tkach/os_info/compare/v1.0.1...v1.0.2
[1.0.0]: https://github.com/stanislav-tkach/os_info/compare/v0.7.0...v1.0
[0.7.0]: https://github.com/stanislav-tkach/os_info/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/stanislav-tkach/os_info/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/stanislav-tkach/os_info/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/stanislav-tkach/os_info/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/stanislav-tkach/os_info/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/stanislav-tkach/os_info/compare/v.0.1.1...v0.2.0
[0.1.1]: https://github.com/stanislav-tkach/os_info/compare/v0.1.0...v.0.1.1
[0.1.0]: https://github.com/stanislav-tkach/os_info/tree/v0.1.0
