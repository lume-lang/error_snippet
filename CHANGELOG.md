# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.14](https://github.com/lume-lang/error_snippet/compare/error_snippet-v0.1.13...error_snippet-v0.1.14) - 2025-07-10

### Added

- *(derive)* added automatic coloring of format args
- *(derive)* added ability to set label severity

### Other

- embed format args in format strings

## [0.1.13](https://github.com/lume-lang/error_snippet/compare/error_snippet-v0.1.12...error_snippet-v0.1.13) - 2025-06-21

### Added

- feat!(derive): allow single related- and causing-error

## [0.1.12](https://github.com/lume-lang/error_snippet/compare/error_snippet-v0.1.11...error_snippet-v0.1.12) - 2025-06-14

### Added

- *(derive)* allow merging spans and labels into single field

## [0.1.11](https://github.com/lume-lang/error_snippet/compare/error_snippet-v0.1.10...error_snippet-v0.1.11) - 2025-06-12

### Other

- updated the following local packages: error_snippet_derive

## [0.1.9](https://github.com/lume-lang/error_snippet/compare/error_snippet-v0.1.8...error_snippet-v0.1.9) - 2025-06-09

### Added

- *(handler)* add `BufferedDiagnosticHandler` for buffered output

## [0.1.8](https://github.com/lume-lang/error_snippet/compare/error_snippet-v0.1.7...error_snippet-v0.1.8) - 2025-05-27

### Added

- add `#[cause]` attribute

## [0.1.7](https://github.com/lume-lang/error_snippet/compare/error_snippet-v0.1.6...error_snippet-v0.1.7) - 2025-05-25

### Other

- *(handler)* add  method to iterate over emitted diagnostics

## [0.1.6](https://github.com/lume-lang/error_snippet/compare/error_snippet-v0.1.5...error_snippet-v0.1.6) - 2025-05-16

### Fixed

- *(handler)* correctly drain diagnostics

## [0.1.5](https://github.com/lume-lang/error_snippet/compare/error_snippet-v0.1.4...error_snippet-v0.1.5) - 2025-05-16

### Added

- feat!(handler): compound errors instead of exiting

## [0.1.4](https://github.com/lume-lang/error_snippet/compare/error_snippet-v0.1.3...error_snippet-v0.1.4) - 2025-05-11

### Added

- add example for handlers

## [0.1.3](https://github.com/lume-lang/error_snippet/compare/error_snippet-v0.1.2...error_snippet-v0.1.3) - 2025-05-11

### Added

- feat!(handler): correctly render diagnostics to stderr

## [0.1.2](https://github.com/lume-lang/error_snippet/compare/error_snippet-v0.1.1...error_snippet-v0.1.2) - 2025-05-11

### Fixed

- fix!(derive): use dynamic errors for #[related]

## [0.1.1](https://github.com/lume-lang/error_snippet/compare/error_snippet-v0.1.0...error_snippet-v0.1.1) - 2025-05-11

### Other

- move repository to `lume-lang`
