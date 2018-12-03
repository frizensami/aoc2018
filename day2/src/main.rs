use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;

const INPUT_FILENAME: &str = "input.txt";

fn main() -> Result<()> {
    println!("Reading AOC Day 1 Input file: input.txt");

    // include_str! is also viable here, but wanted to learn File operations explicitly
    let f = File::open(INPUT_FILENAME)?;

    let mut all_ints: Vec<i32> = Vec::new();

    // Collecting all integers in file
    for line in BufReader::new(f).lines() {
        let line_parse = line.unwrap();
        let sign: i32 = if &(line_parse)[..1] == "-" { -1 } else { 1 }; 
        let val: i32 = (&(line_parse)[1..]).parse().unwrap();
        all_ints.push(sign * val);
    }

    let mut count: i32 = 0;
    let mut unique_counts: HashSet<i32> = HashSet::new();
    let mut idx: usize = 0;

    loop {
        count += all_ints[idx];

        if !unique_counts.insert(count) {
            println!("Duplicate count: {}", count);
            return Ok(());
        } 

        idx = (idx + 1) % all_ints.len();
    }




}
