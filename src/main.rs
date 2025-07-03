use clap::Parser;
use std::path::PathBuf;

#[derive(Debug,Parser)]
struct CLI{
    path:Option<PathBuf>
}
fn main() {
    let cli = CLI::parse();
    println!("Hello, world!");
}
