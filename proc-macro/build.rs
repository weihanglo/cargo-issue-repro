use std::env;

fn main() {
    // We are a proc macro, TARGET should not be set.
    assert!(env::var_os("TARGET").is_none());
}
