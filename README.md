# Guessing Game

This is a simple number guessing game implemented in Rust with both CLI and wasm-targeted UI support.

## Running locally
When running `cargo run` or `cargo test`, the build needs to download crates from crates.io. In restricted environments (such as this sandbox), outbound requests to crates.io receive HTTP 403 responses, which prevents Cargo from fetching dependencies.

### Working around the 403 download error
If you encounter the 403 error during dependency downloads, use one of these approaches from a machine with network access:

1. **Pre-vendor dependencies**
   - Run `cargo vendor --locked --versioned-dirs vendor` on a networked machine to create a `vendor/` directory with all required crates.
   - Commit or copy the `vendor/` directory into this project.
   - Add `.cargo/config.toml` with:
     ```toml
     [source.crates-io]
     replace-with = "vendored-sources"

     [source.vendored-sources]
     directory = "vendor"
     ```
   - Future builds in restricted environments will use the vendored sources without contacting crates.io.

2. **Use an existing local Cargo cache**
   - If your CI or development machine already has a shared Cargo registry/cache, configure Cargo to use it as a mirror or via `CARGO_HOME` so downloads are not needed from the sandbox.

Without one of these workarounds, commands that need to download crates will continue to fail with a 403 due to the environment's network restrictions.
