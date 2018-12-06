extern crate linked_list;

use linked_list::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::vec::Vec;

/*
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum LogEvent {
    WakeUp,
    FallAsleep,
    BeginShift(u32),
}
impl std::fmt::Debug for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#{} {:?}\n", self.id, self.sleep_intervals)
    }
}
*/

const INPUT_FILENAME: &str = "input.2.txt";

/*
    Given a long string of characters, adjacent letters of different cases cancel out.
    The same algorithm is applied to the resultant string until no further actions are possible.
    What is the resultant string given any input string?

    Issue in Rust is that I cannot find a way to manipulate the data structure as I am going through it  (like holding a raw pointer in C) in standard Rust. Would like to avoid copying to new lists multiple times (although functions like .drain() allow the reuse of the space)

    Using external linked-list with "Cursor" API: the Cursor actually points "in-between" two elements, which allows modifications of the list as required

*/
fn main() -> Result<()> {
    println!("Reading AOC Day 5 Input file: input.txt");

    // include_str! is also viable here, but wanted to learn File operations explicitly
    let f = File::open(INPUT_FILENAME)?;
    let mut polymer: LinkedList<char> = LinkedList::new();

    // Reads entire polymer into a string
    let mut polymer_string = String::new();
    BufReader::new(f).read_line(&mut polymer_string)?;

    let mut polymer_cursor = polymer.cursor();

    // This is the forward direction: we keep trying to add chars to the linked list
    // and if we have a "reaction" with a previous element, backtrack.
    let mut previous_char: char = ' ';
    for c in polymer_string.chars() {
        // println!("Input char: {}", c);
        // Continue to loop until no more backwards reactions happen
        if causes_reaction(c, previous_char) {
            // Don't push this char into the list, and remove the previous char
            polymer_cursor.prev();
            polymer_cursor.remove();
            // println!("Reaction: {} and {}", previous_char, c);
            previous_char = *polymer_cursor.peek_prev().unwrap_or(&mut ' ');
        } else {
            polymer_cursor.insert(c);
            polymer_cursor.next();
            // println!("Insert: {}", c);
            previous_char = c;
        }
    }

    polymer_cursor.reset();

    let mut count = -1;
    while let Some(c) = polymer_cursor.next() {
        print!("{}", c);
        count += 1;
    }
    println!();

    println!("Total # chars: {}", count);


    Ok(())
}

fn causes_reaction(a: char, b: char) -> bool {
    // Must be opposite cases, and otherwise equal
    ((a.is_lowercase() && b.is_uppercase()) || (a.is_uppercase() && b.is_lowercase())) && (a.eq_ignore_ascii_case(&b))
}
