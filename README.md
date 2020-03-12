# Investigate Rust Codegen

Setup development environment

```bash
rustup default nightly
rustup target add riscv64gc-unknown-none-elf
cargo install cargo-xbuild cargo-binutils
rustup component add rust-src llvm-tools-preview rustfmt rls rust-analysis
```

objdump

```
RUSTFLAGS="-C target-feature=+a" cargo xbuild --target=riscv64gc-unknown-none-elf
objdump -d target/riscv64gc-unknown-none-elf/debug/libtest_atomic.a > dump.d
```
