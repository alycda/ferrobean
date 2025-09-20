# ferrobean

A Rust Cloudflare Worker project using the official workers-rs template.

## Prerequisites

- Rust toolchain (https://rustup.rs/)
- wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`
- cargo-generate: `cargo install cargo-generate`
- wrangler CLI for deployment: `npm install -g wrangler`

## Development

### Building

```bash
# Build for development
cargo build

# Build for WebAssembly (Cloudflare Workers target)
cargo build --target wasm32-unknown-unknown
```

### Local Development

To run the worker locally for development and testing:

```bash
npx wrangler dev
```

### Deployment

To deploy to Cloudflare Workers:

```bash
npx wrangler deploy
```

## Project Structure

- `src/lib.rs` - Main worker code
- `wrangler.toml` - Cloudflare Workers configuration
- `Cargo.toml` - Rust project configuration

## Generated from Template

This project was generated using:
```bash
cargo generate cloudflare/workers-rs
```

For more information about Cloudflare Workers with Rust, see:
https://developers.cloudflare.com/workers/languages/rust/