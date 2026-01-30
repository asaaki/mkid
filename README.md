# mkid

A tiny Rust-based CLI tool to generate unique IDs, primarily UUIDs. Built as a more capable alternative to `uuidgen`, with support for all UUID versions (v1-v8), multiple output formats, and clipboard integration.

## Features

- **All UUID versions supported**: v1 through v8
- **Smart defaults**: `mkid` generates UUID v7 (sortable, database-friendly)
- **Multiple formats**: hyphenated, simple, URN, braced
- **Case control**: lowercase (default) or uppercase
- **Clipboard integration**: Copy output directly to clipboard
- **Bulk generation**: Generate multiple UUIDs at once
- **Fast and lightweight**: Single binary, minimal dependencies

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
# Binary available at target/release/mkid
```

## Usage

### Basic Usage

Generate a UUID v7 (default):
```bash
mkid
# Output: 019c0edc-d74c-7c02-beb9-40fcf2eee5f6
```

### UUID Versions

Generate specific UUID versions:

```bash
# UUID v4 (random)
mkid uuid v4

# UUID v7 (timestamp-based, sortable)
mkid uuid v7

# UUID v1 (time-based with node ID)
mkid uuid v1 --random-node

# UUID v3 (MD5 hash-based)
mkid uuid v3 --namespace dns --name example.com

# UUID v5 (SHA-1 hash-based)
mkid uuid v5 --namespace url --name https://example.com

# UUID v6 (sortable time-based)
mkid uuid v6 --random-node

# UUID v8 (custom)
mkid uuid v8 --bytes 0123456789abcdef0123456789abcdef
```

### Version-Specific Options

**For v1/v6 (time-based):**
- `--random-node`: Use random 6-byte node ID (default)
- `--node-id <HEX>`: Specify custom node ID (12 hex characters)

```bash
mkid uuid v1 --node-id aabbccddeeff
```

**For v3/v5 (hash-based):**
- `--namespace <NS>`: Namespace (dns, url, oid, x500, or custom UUID)
- `--name <NAME>`: Name to hash (required)

```bash
mkid uuid v5 --namespace dns --name example.com
# Output is deterministic: same namespace + name = same UUID
```

**For v8 (custom):**
- `--bytes <HEX>`: 32 hex characters (16 bytes)

```bash
mkid uuid v8 --bytes 0123456789abcdef0123456789abcdef
```

### Output Formats

```bash
# Hyphenated (default)
mkid uuid v7 --format hyphenated
# 019c0edc-d74c-7c02-beb9-40fcf2eee5f6

# Simple (no hyphens)
mkid uuid v7 --format simple
# 019c0edcd74c7c02beb940fcf2eee5f6

# URN
mkid uuid v7 --format urn
# urn:uuid:019c0edc-d74c-7c02-beb9-40fcf2eee5f6

# Braced
mkid uuid v7 --format braced
# {019c0edc-d74c-7c02-beb9-40fcf2eee5f6}
```

### Case Control

```bash
# Uppercase
mkid uuid v7 --uppercase
# 019C0EDC-D74C-7C02-BEB9-40FCF2EEE5F6

# Lowercase (default)
mkid uuid v7
# 019c0edc-d74c-7c02-beb9-40fcf2eee5f6
```

### Bulk Generation

```bash
# Generate 5 UUIDs
mkid uuid v7 --count 5
# 019c0edd-19ee-7903-8ec0-cc2098967b15
# 019c0edd-19ee-7903-8ec0-cc324e67abd2
# 019c0edd-19ee-7903-8ec0-cc4871c57653
# 019c0edd-19ee-7903-8ec0-cc552b063bff
# 019c0edd-19ee-7903-8ec0-cc6150da757c
```

Note: v7 UUIDs are monotonic (sortable by generation time).

### Clipboard Integration

```bash
# Copy to clipboard
mkid --clipboard
mkid uuid v7 --clipboard

# Copy multiple UUIDs (newline-separated)
mkid uuid v4 --count 5 --clipboard
```

### Combining Options

```bash
# Multiple options at once
mkid uuid v7 --uppercase --format simple --count 3 --clipboard
# 019C0EDD08B37C42AABE599AEA317DE1
# 019C0EDD08B37C42AABE599D2F5B8A92
# 019C0EDD08B37C42AABE599E5C9A1B23
# (also copied to clipboard)
```

## Why UUID v7?

UUID v7 is the default because it provides:

1. **Database performance**: Time-ordered UUIDs reduce B-tree fragmentation (up to 10x better than v4)
2. **Sortability**: Lexicographic comparison works directly on the UUID bytes
3. **Modern timestamps**: Uses Unix epoch in milliseconds (familiar to developers)
4. **Monotonicity**: Sequential UUIDs are guaranteed to be ordered
5. **No clock drift issues**: Better than v1/v6 for distributed systems

## UUID Version Comparison

| Version | Type | Use Case | Deterministic |
|---------|------|----------|---------------|
| v1 | Time + Node | Legacy systems | No |
| v3 | MD5 hash | Namespace-based (legacy) | Yes |
| v4 | Random | General purpose | No |
| v5 | SHA-1 hash | Namespace-based | Yes |
| v6 | Sortable time | Database keys (modern) | No |
| v7 | Timestamp | Database keys (recommended) | No |
| v8 | Custom | Special requirements | Depends |

## Examples

### Generate API keys
```bash
mkid uuid v4 --format simple --uppercase
```

### Generate sortable database IDs
```bash
mkid uuid v7 --count 100
```

### Generate deterministic UUIDs for testing
```bash
mkid uuid v5 --namespace dns --name test-user-1
mkid uuid v5 --namespace dns --name test-user-2
```

### Quick clipboard workflow
```bash
mkid -c  # Generate v7 and copy to clipboard
# Paste anywhere: Cmd+V (macOS) or Ctrl+V (Linux/Windows)
```

## Testing

Run the test suite:

```bash
cargo test
```

The project includes comprehensive integration tests covering:
- All UUID versions
- Format options
- Deterministic behavior (v3/v5)
- Monotonicity (v7)
- Error handling
- Count functionality

## Future Plans

The architecture supports future addition of other ID formats:
- ULID (Universally Unique Lexicographically Sortable Identifier)
- NanoID (URL-friendly unique ID)
- Snowflake (Twitter's distributed ID)
- CUID (Collision-resistant Unique ID)

Example future usage:
```bash
mkid ulid
mkid nanoid
mkid snowflake
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
