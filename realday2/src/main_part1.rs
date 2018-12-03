use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;

const INPUT_FILENAME: &str = "input.txt";

/*
    Parses the input string argument to check if there are any letters that
    - are present exactly two times and/or
    - are present exactly three times 

    Algorithm: 
    - Insert letters and their frequencies into a hashmap
    - Right before we are about to insert a letter into the hashmap, check for 2 conditions
    1. Are we going to change a letter's frequency from 1 -> 2?
        Increment has_freq_two
    2. Are we going to change a letter's frequency from 2 -> 3?
        Increment has_freq_three
        Decrement has_freq_two
    3. Are we going to change a letter's frequency from 3->4?
        Decrement has_freq_three

*/
fn count_2xletter_3xletter(boxid: &str) -> (bool, bool) {
    let (mut freq_two, mut freq_three) = (0, 0);
    let mut char_frequencies: HashMap<char, i32> = HashMap::new();
    for c in boxid.chars() {
        // Not the first time seeing this character
        match char_frequencies.get(&c) {
            None => { char_frequencies.insert(c, 1); }
            Some(1) => { char_frequencies.insert(c, 2); freq_two += 1;}
            Some(2) => { char_frequencies.insert(c, 3); freq_two -= 1; freq_three += 1}
            Some(3) => { char_frequencies.insert(c, 3); freq_three -= 1}
            _ => ()
        }
    }
    
    // Return if we saw at least one of each 
    (freq_two > 0, freq_three > 0)
}

fn main() -> Result<()> {
    println!("Reading AOC Day 2 Input file: input.txt");

    // include_str! is also viable here, but wanted to learn File operations explicitly
    let f = File::open(INPUT_FILENAME)?;

    let (mut num_twos, mut num_threes) = (0, 0);

    // Collecting all integers in file
    for line in BufReader::new(f).lines() {
        let line_parse = line.unwrap();
        let (has_two, has_three) = count_2xletter_3xletter(&line_parse);
        num_twos += if has_two {1} else {0};
        num_threes += if has_three {1} else {0};
    }

    println!("2s: {}, 3s: {}, Checksum: {}", num_twos, num_threes, num_twos * num_threes);

    Ok(())


}
