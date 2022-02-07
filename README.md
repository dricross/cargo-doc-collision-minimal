# Cargo Doc Collision

This repository is a minimal example demonstration of `cargo doc` erroneously
emitting the following error:

```
error: document output filename collision
The lib `child-a-macros` in package `child-a-macros v0.1.0 (/home/dricross/git/cargo-doc-collision-minimal/crates/child-a/macros)` has the same name as the lib `child-a-macros` in package `child-a-macros v0.1.0 (/home/dricross/git/cargo-doc-collision-minimal/crates/child-a/macros)`.
Only one may be documented at once since they output to the same path.
Consider documenting only one, renaming one, or marking one with `doc = false` in Cargo.toml.
```

This is a demonstration monorepo which contains a proc-macro crate. Key configurations that appear to cause this issue:
* Virtual workspace containing a crate containing a proc-macro crate.
* Target specified in workspace configuration file.
* Any level of optimizations enabled.
* Certain compiler verions used, starting at least as early as `nightly-2021-10-14`.

## Notes

* Removing the `[build]` section and running `cargo doc --target x86_64-unknown-linux-gnu` will succeed.
* Changing `opt-level` in `dev` profile to 0 will cause `cargo doc` to succeed. `cargo doc --release` will continue to fail.
   * Any non-zero `opt-level` will fail.
* rustc version `nightly-2021-10-14` is the first to display this behavior for
`x86_64-unknown-linux-gnu` and `armv7r-none-eabihf` targets. Prior nightly rustc
versions will display this behavior for `x86_64-apple-darwin`.

