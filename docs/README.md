# Documentation

This project uses GitHub Pages to automatically generate and publish Rust documentation.

## How it works

1. On every push to the `main` branch, the CI workflow generates documentation using `cargo doc`
2. The generated documentation is automatically deployed to GitHub Pages
3. The documentation is available at `https://<username>.github.io/ferrobean/`

## Accessing Documentation

- **Main documentation**: Navigate to the GitHub Pages URL for this repository
- **Local documentation**: Run `cargo doc --open` to generate and view docs locally

## CI Integration

The documentation deployment is integrated into the existing CI workflow in `.github/workflows/ci.yml`:
- Tests must pass before documentation is generated
- Documentation is only generated and deployed on pushes to the main branch
- Uses GitHub's official Pages actions for reliable deployment