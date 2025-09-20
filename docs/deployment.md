# Deployment

Learn how to deploy Ferrobean to Cloudflare Workers.

## Prerequisites

Before deploying, ensure you have:

- A Cloudflare account
- Wrangler CLI installed and configured
- The project built for the WebAssembly target

## Configuration

The deployment configuration is defined in `wrangler.toml`:

```toml
name = "ferrobean"
compatibility_date = "2023-12-07"
main = "build/worker/shim.mjs"

[build]
command = "cargo install -q worker-build && worker-build --release"
```

## Deployment Steps

### 1. Build for Production

```bash
cargo build --target wasm32-unknown-unknown --release
```

### 2. Deploy to Cloudflare Workers

```bash
npx wrangler deploy
```

This command will:
- Build the worker using the configuration in `wrangler.toml`
- Upload the compiled WebAssembly to Cloudflare
- Deploy to your Cloudflare Workers subdomain

### 3. Verify Deployment

After deployment, test your worker:

```bash
curl https://your-worker.your-subdomain.workers.dev/
```

## Environment Variables

Configure environment-specific settings through Wrangler:

```bash
# Set environment variables
npx wrangler secret put SECRET_NAME
```

## Monitoring

Monitor your deployment through:

- Cloudflare Workers dashboard
- Wrangler analytics: `npx wrangler tail`
- Application logs and metrics

## Rollback

If you need to rollback a deployment:

```bash
# View deployment history
npx wrangler deployments list

# Rollback to a specific deployment
npx wrangler rollback [deployment-id]
```

## Custom Domains

To use a custom domain:

1. Add the domain to your Cloudflare account
2. Configure DNS settings
3. Add routes in `wrangler.toml`:

```toml
[[routes]]
pattern = "api.yourdomain.com/*"
zone_name = "yourdomain.com"
```

## Best Practices

- Always test in staging before production deployment
- Monitor performance and error rates
- Use environment variables for configuration
- Implement proper error handling and logging
- Keep deployment artifacts minimal for faster cold starts