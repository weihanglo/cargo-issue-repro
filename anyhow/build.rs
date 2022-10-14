use std::env;

fn main() {
    println!("cargo:warning=anyhow TARGET: {}", env::var("TARGET").unwrap());
    println!("cargo:warning=anyhow HOST:   {}", env::var("HOST").unwrap());
}
