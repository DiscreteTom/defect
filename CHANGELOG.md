# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Streaming output for OpenAI (or compatible) models.

## [0.2.0] - 2025-03-01

### Added

- Add pre-built static-linked binaries for x86_64 linux, aarch64 linux and x86_64 windows.
- Add CLI option `schema`.

### Changed

- To call AWS Bedrock, the CLI option `schema` is required.
- Rename environment variable `API_KEY` to `DEFECT_API_KEY`.

## [0.1.0] - 2025-02-23

### Added

- Add `Invoker`, `OpenAIInvoker`, `BedrockInvoker`, `OpenAIConfig`, `BedrockConfig`.
- Add a binary executable `defect`.

[unreleased]: https://github.com/DiscreteTom/defect/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/DiscreteTom/defect/releases/tag/v0.2.0
[0.1.0]: https://github.com/DiscreteTom/defect/releases/tag/v0.1.0
