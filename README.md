# ASIMOV X Module

[![Package on Crates.io](https://img.shields.io/crates/v/asimov-x-module)](https://crates.io/crates/asimov-x-module)
[![Documentation](https://docs.rs/asimov-x-module/badge.svg)](https://docs.rs/asimov-x-module)

ASIMOV module for importing data from X (Twitter) lists and converting it to JSON-LD format.

## ✨ Features

🐦 **Import List Members**: Fetch and convert X list members to JSON-LD
 **User Profiles**: Complete user data including metrics and profile information
🔄 **JSON-LD Output**: Structured data compatible with KNOW ontology
⚡ **Fast & Reliable**: Built with Rust for performance and safety
 **Rate Limit Aware**: Respects X API limits and provides clear error handling
📊 **Flexible Output**: Support for both JSON and JSONL formats
🔢 **Pagination Control**: Limit results with `--limit` option

## 🛠️ Prerequisites

- Rust 1.85+ (2024 edition) if building from source code
- X (Twitter) developer account with API access
- `X_TOKEN` environment variable

## ⬇️ Installation

### Installation from Package Managers

```bash
# From Cargo (Rust)
cargo install asimov-x-module

# From source
git clone https://github.com/asimov-modules/asimov-x-module.git
cd asimov-x-module
cargo build --release
```

## 👉 Examples

### Import List Members

```bash
# Import all list members (default JSONL output)
asimov-x-cataloger get-list-members "https://x.com/i/lists/${random_string}"

# Import first 100 members only
asimov-x-cataloger get-list-members "https://x.com/i/lists/${random_string}" --limit 100

# Output in JSON format instead of JSONL
asimov-x-cataloger get-list-members "https://x.com/i/lists/${random_string}" --limit 100 --output json
```

### Other Commands

```bash
# Show version information
asimov-x-cataloger --version

# Show license information
asimov-x-cataloger --license

# Show help
asimov-x-cataloger --help
```

## ⚙️ Configuration

### API Token Setup

Set your X API token as an environment variable:

```bash
export X_TOKEN="your-x-api-token-here"
```

Or use a `.env` file:

```bash
X_TOKEN=your-x-api-token-here
```

Get your API token from: [X Developer Portal](https://developer.x.com/)

### Rate Limiting

Heads up: This module plays nice with X's API limits. If you get HTTP 429 errors (too many requests):

- **Just wait a bit**: The API will throw an error if you're going too fast
- **No auto-retry**: We don't automatically retry failed requests  
- **Take it slow**: If you're grabbing lots of data, give it some breathing room

The limits: X API has rate limits that vary by endpoint and authentication level.

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-x-module.git
cd asimov-x-module
cargo test
cargo build --bin asimov-x-cataloger
```

## 📊 Output Formats

### JSONL (Default)

```jsonl
{"@type":"know:XUser","id":"123","name":"John Doe","username":"johndoe",...}
{"@type":"know:XUser","id":"456","name":"Jane Smith","username":"janesmith",...}
```

### JSON (Structured)

```json
{
  "@context": {...},
  "@id": "https://x.com/i/lists",
  "members": {
    "count": 2,
    "items": [...]
  }
}
```

## Links

- [ASIMOV]: https://github.com/asimov-modules
- [JSON-LD]: https://json-ld.org/
- [X API]: https://developer.x.com/

## 📄 License

This project is licensed under the Unlicense - see the [UNLICENSE](UNLICENSE) file for details.
