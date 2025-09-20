# Project Structure

Understanding the Ferrobean codebase organization and architecture.

## Repository Layout

```
ferrobean/
├── .github/
│   └── workflows/
│       └── ci.yml          # CI/CD workflow
├── docs/                   # Documentation source
├── src/                    # Rust source code
│   ├── lib.rs             # Main entry point
│   └── beans/             # Beancount processing modules
│       ├── mod.rs         # Module declarations
│       └── flags.rs       # Transaction flags implementation
├── book.toml              # mdbook configuration
├── Cargo.toml             # Rust project configuration
├── Cargo.lock             # Dependency lock file
├── wrangler.toml          # Cloudflare Workers configuration
├── README.md              # Project overview
└── .gitignore            # Git ignore rules
```

## Source Code Organization

### Main Entry Point (`src/lib.rs`)

The main library file contains:

- Worker request handler function
- Core error types (`Helpers` enum)
- Module declarations
- WebAssembly compilation target configuration

### Beans Module (`src/beans/`)

The beans module handles beancount-specific functionality:

- **`flags.rs`**: Transaction flag parsing and validation
  - Enum definition for all beancount flags
  - String and byte conversion implementations
  - Comprehensive test coverage

## Configuration Files

### `Cargo.toml`

Rust project configuration including:
- Package metadata
- Dependencies (worker, console_error_panic_hook)
- Library type configuration for WebAssembly

### `wrangler.toml`

Cloudflare Workers deployment configuration:
- Worker name and compatibility date
- Build commands and entry point
- Runtime settings

### `book.toml`

mdbook configuration for documentation:
- Book metadata (title, authors, language)
- HTML output settings
- Search and navigation configuration

## Data Structures

### Transaction Flags

The `Flags` enum represents beancount transaction flags:

```rust
#[repr(u8)]
pub(crate) enum Flags {
    Conversion = b'C',
    Merging = b'M',
    Okay = b'*',
    Padding = b'P',
    Returns = b'R',
    Summarize = b'S',
    Transfer = b'T',
    Unrealized = b'U',
    Warning = b'!',
}
```

Features:
- Memory-efficient byte representation
- Bidirectional string conversion
- Case-insensitive parsing
- Comprehensive error handling

## Build Artifacts

### Development

- `target/debug/` - Debug builds
- `target/wasm32-unknown-unknown/debug/` - WebAssembly debug builds

### Production

- `target/release/` - Optimized builds
- `target/wasm32-unknown-unknown/release/` - Production WebAssembly builds

## Testing Structure

Tests are co-located with the source code using Rust's built-in testing framework:

```rust
#[cfg(test)]
mod tests {
    // Test implementations
}
```

Current test coverage includes:
- Flag parsing and validation
- String/byte conversions
- Error handling scenarios