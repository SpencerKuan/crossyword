use std::fs;

mod generate_layout;
use crate::generate_layout::*;

fn main() -> std::io::Result<()> {
    let unformatted_words = fs::read_to_string("./words.txt")?;

    let formatted_words = format_words(&unformatted_words);

    let puzzle = new_puzzle(&formatted_words).unwrap();
    println!("{:?}", puzzle);

    Ok(())
}

