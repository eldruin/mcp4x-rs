# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [1.0.0] - 2024-06-24

### Changed
- Updated embedded-hal to 1.0
- Updated MSRV to Rust 1.62.0.

## [0.2.0] - 2021-09-23

### Changed
- [breaking-change] Use fallible output pins. `Error` type now contains `Pin` variant.
- `interface` module is now public to ease usage.

## 0.1.0 - 2019-02-09

This is the initial release to crates.io. All changes will be documented in
this CHANGELOG.

<!-- next-url -->
[Unreleased]: https://github.com/eldruin/mcp4x-rs/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/eldruin/mcp4x-rs/compare/v0.2.0...v1.0.0
[0.2.0]: https://github.com/eldruin/mcp4x-rs/compare/v0.1.0...v0.2.0