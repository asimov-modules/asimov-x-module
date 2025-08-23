# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.2 - 2025-08-23

### Changed

- Fixed manifest

## [0.2.1] - 2025-08-23

### Changed

- **CLI Simplification**: Removed subcommand structure for cleaner command interface
- **Command Usage**: Changed from `asimov-x-cataloger get-list-members "URL"` to `asimov-x-cataloger "URL"`
- **Documentation**: Updated README.md and module.yaml to reflect new CLI structure
- **Module Configuration**: Updated .asimov/module.yaml to remove subcommand definitions

### Breaking Changes

- **CLI Interface**: The `get-list-members` subcommand has been removed
- **Command Syntax**: URL is now a direct positional argument instead of requiring a subcommand

### Migration

**Old usage:**

```bash
asimov-x-cataloger get-list-members "https://x.com/i/lists/1234567890" --limit 100
```

**New usage:**

```bash
asimov-x-cataloger "https://x.com/i/lists/1234567890" --limit 100
```

## [0.1.0] - 2025-08-21

### Changed

- **Code Formatting**: Improved code formatting and code organization
- **Dependencies**: Cleaned up Cargo.toml and removed unused tracing-subscriber
- **Documentation**: Updated README.md and CHANGES.md formatting

## [0.0.1] - 2025-08-21

### Added

- **New ASIMOV X Module** - dedicated to X (Twitter) platform integration
- X (Twitter) list cataloger implementation
- X API v2 client with Bearer token authentication
- X list members data extraction and processing
- JQ query support for JSON-LD transformation
- CLI interface with subcommand structure (`get-list-members`)
- Support for `--limit` and `--output` (json/jsonl) options
- Provider framework for X platform URL pattern matching
- JSON-LD output format with semantic annotations using know.dev vocabulary
- Error handling for API rate limits (429) and authentication (401)
- Comprehensive test coverage for API endpoints and JQ transformations
- Support for X list URLs: `https://x.com/i/lists/{list_id}`

### Features

- **Authentication**: Uses `X_TOKEN` environment variable for API access
- **Data Processing**: Fetches list members with full user profiles and metrics
- **Output Formats**:

  - JSONL: One user per line (default, pipe-friendly)
  - JSON: Complete structured data with semantic context

- **Rate Limiting**: Handles X API rate limits gracefully
- **Error Handling**: Clear error messages for common API issues

