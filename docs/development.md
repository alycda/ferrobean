# Development

This section covers the development workflow and architecture of Ferrobean.

## Project Architecture

Ferrobean is structured as a Rust library that compiles to WebAssembly for deployment on Cloudflare Workers.

### Core Components

- **Worker Handler**: Main request/response processing logic
- **Beans Module**: Beancount-specific data structures and processing
- **Flags System**: Transaction flag parsing and validation

## Development Workflow

### Building

```bash
# Standard Rust build
cargo build

# WebAssembly build for deployment
cargo build --target wasm32-unknown-unknown
```

### Testing

Run the test suite to ensure everything works correctly:

```bash
cargo test
```

The project includes unit tests for core functionality, particularly the flags parsing system.

### Local Development Server

Use Wrangler to run a local development server:

```bash
npx wrangler dev
```

This provides hot reloading and local testing capabilities.

## Code Organization

```
src/
├── lib.rs          # Main worker entry point and request handler
├── beans/          # Beancount-specific modules
│   ├── mod.rs      # Module declarations
│   └── flags.rs    # Transaction flags implementation
```

### Key Files

- **`src/lib.rs`**: Contains the main worker handler and error types
- **`src/beans/flags.rs`**: Implements beancount transaction flags with comprehensive parsing and validation
- **`wrangler.toml`**: Cloudflare Workers configuration
- **`Cargo.toml`**: Rust project configuration

## Adding New Features

1. Implement your feature in the appropriate module under `src/`
2. Add comprehensive tests
3. Update documentation
4. Test locally with `wrangler dev`
5. Deploy to staging for integration testing

## Error Handling

Ferrobean uses a custom error type system:

```rust
enum Helpers {
    BeancountError(String),
    FavaError(String)
}
```

Ensure all new code follows this error handling pattern for consistency.