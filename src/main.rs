#![allow(unused)] // For beginning only.

use crate::prelude::*;
use std::fs::read_dir;

mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    println!("Hello, world!");

    // Example of the newtype pattern
    for entry in read_dir("./")?.filter_map(std::result::Result::ok) {
        let entry: String = W(&entry).try_into()?;
        println!("{entry}");
    }

    Ok(())
}
