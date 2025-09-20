# Getting Started

This guide will help you get started with Ferrobean development and usage.

## Prerequisites

Before working with Ferrobean, ensure you have the following installed:

- **Rust toolchain**: Install from [https://rustup.rs/](https://rustup.rs/)
- **WebAssembly target**: `rustup target add wasm32-unknown-unknown`
- **cargo-generate**: `cargo install cargo-generate`
- **Wrangler CLI**: `npm install -g wrangler` (for deployment)

## Installation

### Clone the Repository

```bash
git clone https://github.com/alycda/ferrobean.git
cd ferrobean
```

### Build the Project

```bash
# Build for development
cargo build

# Build for WebAssembly (Cloudflare Workers target)
cargo build --target wasm32-unknown-unknown
```

## Quick Start

### Local Development

To run the worker locally for development and testing:

```bash
npx wrangler dev
```

This will start a local development server where you can test your worker.

### Basic Usage

Once running, you can interact with the Ferrobean API:

```bash
curl http://localhost:8787/
```

Expected response:
```
Hello World!
```

## Next Steps

- Read the [Development](./development.md) guide to learn about the codebase
- Check the [API Reference](./api-reference.md) for detailed endpoint documentation
- See [Deployment](./deployment.md) for production deployment instructions