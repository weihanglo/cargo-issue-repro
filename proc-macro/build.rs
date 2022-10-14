use std::env;

fn main() {
    println!("cargo:warning=pm TARGET: {}", env::var("TARGET").unwrap());
    println!("cargo:warning=pm HOST:   {}", env::var("HOST").unwrap());
}
