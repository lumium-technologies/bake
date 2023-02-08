use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    recipes: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    Ok(())
}
