# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.3] - 2025-03-15

### Added

- Auto correct the `OPENAI_API_BASE` if it doesn't ends with `/`.

## [0.3.2] - 2025-03-13

### Added

- Better error output for OpenAI models.

## [0.3.1] - 2025-03-02

### Added

- New CLI option `-S/--system` to specify system instructions.

### Changed

- The pre-built binaries are zipped.

## [0.3.0] - 2025-03-01

### Added

- Streaming output for OpenAI (or compatible) models.

### Changed

- OpenAI client will read the api key from `OPENAI_API_KEY` instead of `DEFECT_API_KEY`.
- OpenAI client will read the endpoint overrides from environment variable `OPENAI_API_BASE` instead of CLI option.
- Changed the default schema name to `openai` instead of `open-ai`.

### Removed

- Remove the `endpoint` CLI option.

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

[unreleased]: https://github.com/DiscreteTom/defect/compare/v0.3.3...HEAD
[0.3.3]: https://github.com/DiscreteTom/defect/releases/tag/v0.3.3
[0.3.2]: https://github.com/DiscreteTom/defect/releases/tag/v0.3.2
[0.3.1]: https://github.com/DiscreteTom/defect/releases/tag/v0.3.1
[0.3.0]: https://github.com/DiscreteTom/defect/releases/tag/v0.3.0
[0.2.0]: https://github.com/DiscreteTom/defect/releases/tag/v0.2.0
[0.1.0]: https://github.com/DiscreteTom/defect/releases/tag/v0.1.0
