use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILENAME: &str = "input.txt";

fn main() -> Result<()> {
    println!("Reading AOC Day 1 Input file: input.txt");

    // include_str! is also viable here, but wanted to learn File operations explicitly
    let f = File::open(INPUT_FILENAME)?;

    let mut count: i32 = 0;
    // Iteratng through each line in file
    for line in BufReader::new(f).lines() {
        let line_parse = line.unwrap();
        let sign: i32 = if &(line_parse)[..1] == "-" { -1 } else { 1 }; 
        let val: i32 = (&(line_parse)[1..]).parse().unwrap();
        count += sign * val;

        println!("Line: {}, sign: {}, val: {}", line_parse, sign, val);
    }

    println!("Result: {}", count);

    Ok(())
}
