# Change Log

All notable changes to this project will be documented in this file.

## [Unreleased]

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

- Use release files is possible (#48).

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

[Unreleased]: https://github.com/darkeld3r/os_info/compare/v1.3.2...HEAD
[1.3.2]: https://github.com/darkeld3r/os_info/compare/v1.3.1...v1.3.2
[1.3.1]: https://github.com/darkeld3r/os_info/compare/v1.3...v1.3.1
[1.3.0]: https://github.com/darkeld3r/os_info/compare/v1.2...v1.3
[1.2.0]: https://github.com/darkeld3r/os_info/compare/v1.1.3...v1.2
[1.1.3]: https://github.com/darkeld3r/os_info/compare/v1.1.2...v1.1.3
[1.1.2]: https://github.com/darkeld3r/os_info/compare/v1.1.1...v1.1.2
[1.1.1]: https://github.com/darkeld3r/os_info/compare/v1.1...v1.1.1
[1.1.0]: https://github.com/darkeld3r/os_info/compare/v1.0.3...v1.1
[1.0.3]: https://github.com/darkeld3r/os_info/compare/v1.0.2...v1.0.3
[1.0.2]: https://github.com/darkeld3r/os_info/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/darkeld3r/os_info/compare/v1.0.1...v1.0.2
[1.0.0]: https://github.com/darkeld3r/os_info/compare/v0.7.0...v1.0
[0.7.0]: https://github.com/darkeld3r/os_info/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/darkeld3r/os_info/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/darkeld3r/os_info/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/darkeld3r/os_info/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/darkeld3r/os_info/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/darkeld3r/os_info/compare/v.0.1.1...v0.2.0
[0.1.1]: https://github.com/darkeld3r/os_info/compare/v0.1.0...v.0.1.1
[0.1.0]: https://github.com/darkeld3r/os_info/tree/v0.1.0
