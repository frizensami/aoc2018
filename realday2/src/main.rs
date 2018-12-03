use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::vec::Vec;

const INPUT_FILENAME: &str = "input.txt";
/*
    Problem: We need to find the two words in a set of words such that only differ by 1 letter
    AT THE SAME POSITION (ccc vs ccd)

    Approach 0: Check every possible pair of words: O(n^2 * k), n = total number of words, k = word length. 

    Approach 1: Sum up their word values and check those that have a difference of only 1
    Problem: ccc and bce have relative values (0, +1), but differ in 2 locations. 
    So worst case: we have a lot of words with that exact difference. 
    Approach 1 is --not significantly better--


*/
fn main() -> Result<()> {
    println!("Reading AOC Day 2 Input file: input.txt");

    // include_str! is also viable here, but wanted to learn File operations explicitly
    let f = File::open(INPUT_FILENAME)?;

    let mut boxids: Vec<String> = Vec::new();

    // Collecting all strings in the file for futher processing 
    for line in BufReader::new(f).lines() {
        let line_parse = line.unwrap();
        boxids.push(line_parse.clone());
    }

    for (i, item) in boxids.iter().enumerate() {
        if let Some(valid_str) = has_hamming_distance_one(&item, &boxids[i+1..]) {
            println!("Match: \n{}\n{}", valid_str, item);
            println!("Common characters: {}", common_characters(&valid_str, item))
        }
    }


    Ok(())
}

fn has_hamming_distance_one(string_to_match: &str, rest_of_strings: &[String]) -> Option<String> {
    for s in rest_of_strings {
        if hamming_distance_same_len(string_to_match, s) == 1 {
            return Some(s.clone());
        }
    }
    None
}

fn hamming_distance_same_len(x: &str, y: &str) -> u32 {
    assert_eq!(x.len(), y.len());
    let dist: u32 = x.chars().zip(y.chars()).map(|(c1, c2)| if c1 == c2 {0} else {1}).sum();
    dist
}

fn common_characters(x: &str, y: &str) -> String {
    x.chars().zip(y.chars()).filter(|(c1, c2)| c1 == c2).map(|(c1, _)| c1).collect::<String>()
}
