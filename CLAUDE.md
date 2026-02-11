# CLAUDE.md

Personal portfolio website for Spencer C. Imbleau, built with Rust + Yew compiled to WebAssembly.
Live at <https://spencer.imbleau.com>.

## Build & Run

```bash
trunk serve --open          # Dev server with hot reload (localhost:8080)
trunk build --release       # Production build → dist/
```

## Lint, Format, Check

```bash
cargo clippy -- -D warnings                   # Lint (warnings are errors)
cargo fmt --all -- --check                     # Check formatting
cargo fmt --all                                # Auto-format
cargo check --target wasm32-unknown-unknown    # Type check
```

## Project Structure

- `src/main.rs` — Entry point, app initialization, theme management
- `src/router.rs` — Routes: Home, Resume, 404, Error
- `src/components/` — Reusable UI components (header, nav, icons, links, footer)
- `src/page_components/` — Full page components (home, resume, not_found, server_error)
- `src/hooks/` — Custom hooks (use_theme)
- `src/style/` — Global styles and theme definitions
- `static/` — Images, fonts, favicons, manifest, robots.txt, sitemap
- `terraform/` — AWS/Cloudflare infrastructure (S3, CloudFront, ACM, DNS)

## Key Conventions

- **Rust edition 2024**, toolchain 1.90.0, target `wasm32-unknown-unknown`
- **Formatting**: rustfmt with 100 char max width, Unix newlines, item-level imports (see `rustfmt.toml`)
- **Styling**: Stylist CSS-in-JS co-located with components
- **Modules**: use `mod.rs` pattern with `pub use` re-exports
- **License**: Dual MIT / Apache-2.0

## CI (GitHub Actions)

- **ci.yml**: Runs on push/PR — Terraform validate, Clippy + fmt check, cargo check
- **release.yml**: Manual dispatch — Terraform apply, trunk build, deploy to S3, CloudFront invalidation

## Dependencies

Core: yew 0.21, yew-router 0.18, stylist 0.13, wasm-bindgen, web-sys, gloo-*
