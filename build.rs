use std::env;

fn main() {
    println!("cargo:warning=root TARGET: {}", env::var("TARGET").unwrap());
    println!("cargo:warning=root HOST:   {}", env::var("HOST").unwrap());
}
