# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-08-28

[0.3.0]: https://github.com/sunsided/basic-dsl/releases/tag/v0.3.0

### Added

- **INPUT statement** for interactive user input with automatic type detection:
  - `INPUT variable` for simple input prompting with "?"
  - `INPUT "prompt", variable` for custom prompts
  - Automatic parsing of numeric vs string input
  - Seamless integration with existing variables and expressions
- Added number guessing game example demonstrating binary search algorithm with INPUT/OUTPUT
- Added comprehensive INPUT test example with various scenarios

## [0.2.0] - 2025-08-28

[0.2.0]: https://github.com/sunsided/basic-dsl/releases/tag/v0.2.0

### Added

- Added RustBasic FitzBuzz example demonstrating classic BASIC programming patterns with Rust-themed output.
- Enhanced PRINT statement with multiple expression support and authentic BASIC separators:
  - Comma (`,`) separator creates tab stops at 14-character boundaries for columnar output
  - Semicolon (`;`) separator concatenates expressions without spacing
  - Support for mixed separators in single PRINT statement
  - Empty PRINT statements for newlines

### Changed

- **BREAKING**: Replaced string-based parsing with direct token parsing for better IDE support and performance. BASIC code is now written directly in the macro body without string quotes.
- **BREAKING**: PRINT statement now accepts multiple comma-separated or semicolon-separated expressions instead of single expression.

### Removed

- **BREAKING**: Removed support for string literal syntax in `basic!` macro. Use direct token syntax instead.

## [0.1.1] - 2025-09-27

[0.1.1]: https://github.com/sunsided/basic-dsl/releases/tag/v0.1.1

### Added

- Added `basic!` macro to interpret and exectue a string of BASIC code.
