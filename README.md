# foo 

To make cargo clippy work cargo build clippy and 

```
~/src/rust-clippy (master)$ mv target/debug/clippy-driver  /home/ysz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/
~/src/rust-clippy (master)$ mv target/debug/cargo-clippy  /home/ysz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/
```

How to run clippy

    cargo +nightly clippy -- -D warnings
