# ASIMOV X Module

[![Package on Crates.io](https://img.shields.io/crates/v/asimov-x-module)](https://crates.io/crates/asimov-x-module)
[![Documentation](https://docs.rs/asimov-x-module/badge.svg)](https://docs.rs/asimov-x-module)

ASIMOV module for importing data from X (Twitter) lists and converting it to JSON-LD format.

## ✨ Features

🐦 **Import List Members**: Fetch and convert X list members to JSON-LD
🔄 **User Profiles**: Complete user data including metrics and profile information
🔄 **JSON-LD Output**: Structured data compatible with KNOW ontology
⚡ **Fast & Reliable**: Built with Rust for performance and safety
📊 **Rate Limit Aware**: Respects X API limits and provides clear error handling
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
```

## 👉 Examples

### Import List Members

```bash
# Import all list members (default JSONL output)
asimov-x-cataloger "https://x.com/i/lists/1234567890"

# Import first 100 members only
asimov-x-cataloger "https://x.com/i/lists/1234567890" --limit 100

# Output in JSON format instead of JSONL
asimov-x-cataloger "https://x.com/i/lists/1234567890" --limit 100 --output json
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

⚠️ **IMPORTANT**: X API has strict rate limits that vary by subscription plan.

#### **Free Plan**

- **Maximum users per request**: 100 users only
- **Request frequency**: 1 request every 15 minutes
- **Error**: Returns HTTP 429 if you try within 15-minute window
- **Pagination**: Limited to single page (100 users max)

#### **Basic Plan**

- **Maximum users per request**: 100 users
- **Request frequency**: 5 requests per 15 minutes (per user)
- **App-wide limits**: 25 requests per 15 minutes (per app)
- **Pagination**: Can handle multiple pages with delays

#### **Pro Plan**

- **Maximum users per request**: 100 users
- **Request frequency**: 900 requests per 15 minutes (per user)
- **App-wide limits**: 900 requests per 15 minutes (per app)
- **Pagination**: Can handle large lists efficiently

**Free plan users**: Use `--limit=100` and wait 15 minutes between requests.

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
