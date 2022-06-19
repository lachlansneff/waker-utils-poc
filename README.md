# Proof of Concept for a `waker-utils` crate for doing async executor specific optimizations 

- Open `a-binary/Cargo.toml`
- Run `cargo r --manifest-path a-binary/Cargo.toml`
- In `a-binary/Cargo.toml`, comment out line 12  and uncomment line 14
- Run `cargo r --manifest-path a-binary/Cargo.toml` again
- Notice that the Waker type used by `a-driver` changes.
