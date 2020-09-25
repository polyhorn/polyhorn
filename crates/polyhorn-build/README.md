# Polyhorn Build

This repository contains code for the `polyhorn-build` crate that
automatically finds, compiles and links native code for iOS (and other platforms
in the future).

This is probably only useful if you're using [Polyhorn](https://polyhorn.com/).

## Usage

Create a new `build.rs` file:

```rust
fn main() {
    polyhorn_build::build();
}
```

Polyhorn Build will figure out the rest!