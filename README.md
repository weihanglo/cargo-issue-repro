# Cargo Reproducible Issues

To reproduce, run:

```console
cargo b --target wasm32-wasi -j1
```

You're expected to see:

```console
   Compiling anyhow v0.1.0 (/Users/whlo/wd/cargo-issue-repro/anyhow)
warning: anyhow TARGET: aarch64-apple-darwin
warning: anyhow HOST:   aarch64-apple-darwin
   Compiling autocfg v1.1.0
   Compiling the_macro v0.1.0 (/Users/whlo/wd/cargo-issue-repro/proc-macro)
warning: pm TARGET: aarch64-apple-darwin
warning: pm HOST:   aarch64-apple-darwin
   Compiling cargo-miri-test v0.1.0 (/Users/whlo/wd/cargo-issue-repro)
warning: root TARGET: wasm32-wasi
warning: root HOST:   aarch64-apple-darwin
    Finished dev [unoptimized + debuginfo] target(s) in 1.76s
```

## Version Info

```
cargo 1.66.0-nightly (b8f30cb23 2022-10-10)
release: 1.66.0-nightly
commit-hash: b8f30cb23c4e5f20854a4f683325782b7cff9837
commit-date: 2022-10-10
host: aarch64-apple-darwin
libgit2: 1.5.0 (sys:0.15.0 vendored)
libcurl: 7.79.1 (sys:0.4.55+curl-7.83.1 system ssl:(SecureTransport) LibreSSL/3.3.6)
os: Mac OS 12.6.0 [64-bit]
```
