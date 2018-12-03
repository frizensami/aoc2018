use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::vec::Vec;
use std::i32;

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

    let mut map: Vec<(String, u32)> = Vec::new();

    // Collecting all integers in file
    for line in BufReader::new(f).lines() {
        let line_parse = line.unwrap();
        let value: u32 = line_parse.chars().map(|c| c as u32 ).sum();
        map.push((line_parse.clone(), value));
    }

    // Sort the map
    map.sort_by(|(_, a_val), (_, b_val) | a_val.cmp(b_val));

    let mut prev_line = "";
    let mut prev_value = std::u32::MAX;
    for (line, value) in &map {
        //println!("Line: {}, Val: {}", line, value);
        // We want to make sure there is only a delta of 1 between previous and current values
        // This must mean we only diff by 1 character value
        if i32::abs((prev_value as i32) - (*value as i32)) == 1 {
            //println!("Differing by 1 char:\n{}\n{}", prev_line, line);

            // Now, we could have constructed a worst-case scenario with all words
            // summing to an absolute difference of 1, but this should vastly speed up general cases

            // Now to check if the strings differ at only spot
            // Sanity check to ensure they are of the same length
            if prev_line.len() == line.len() {
                let zipped_chars = prev_line.chars().zip(line.chars());
                let non_repeated = zipped_chars.filter(|(c1, c2)| c1 == c2).map(|(c3, _)| c3).collect::<String>();

                // Only differs by 1 char
                if non_repeated.len() == line.len() - 1 {
                    println!("!!!SAME LETTERS: {}!!!", non_repeated);
                } else {
                    //println!("Differ by {} chars, but SAME LETTERS: {}", line.len() - non_repeated.len(), non_repeated);
                }
            }

            println!();
        }

        prev_line = line;
        prev_value = *value;



    }


    Ok(())


}
