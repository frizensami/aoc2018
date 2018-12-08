extern crate petgraph;
extern crate binary_heap_plus;

use petgraph::algo::toposort;
use petgraph::dot::{Config, Dot};
use petgraph::graphmap::DiGraphMap;
use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::vec::Vec;
use binary_heap_plus::*;

const INPUT_FILENAME: &str = "input.txt";
fn main() -> Result<()> {
    println!("Reading AOC Day 7 Input file: input.txt");

    let f = File::open(INPUT_FILENAME)?;
    let mut g = DiGraphMap::new();

    // Parse all log entries
    for line in BufReader::new(f).lines() {
        let line_parse = line.unwrap();
        let (parent, child): (char, char) = line_to_edge(&line_parse);
        println!("{} -> {}", parent, child);
        g.add_edge(parent, child, -1);
    }

    let mut visited: HashSet<char> = HashSet::new();
    // Add all neighbors to the frontier: will be sorted by char
    // println!("{:?}", cur_satisfied_nodes);
    
    loop {
        let mut cur_satisfied_nodes = find_all_prereq_satisfied_alphabetical(&g, &mut visited);
        if let Some(cur_node) = cur_satisfied_nodes.pop() {
            // println!("Cur node {:?}, cur satisfied: {:?}", cur_node, cur_satisfied_nodes);
            visited.insert(cur_node);
            print!("{}", cur_node);
        } else {
            println!("\nAll satified");
            break;
        }
    }

    println!();

    Ok(())
}

fn line_to_edge(line: &str) -> (char, char) {
    (line.chars().nth(5).unwrap(), line.chars().nth(36).unwrap())
}

// Search all nodes for those that have their parent nodes visited
fn find_all_prereq_satisfied_alphabetical(g: &DiGraphMap<char, i8>, visited: &mut HashSet<char>) -> Vec<char> {
    let all_nodes = g.nodes();
    let all_nodes_prereqs = all_nodes.map(|n| (n, g.neighbors_directed(n, petgraph::Direction::Incoming).collect()));
    let mut satisfied_nodes: Vec<char> = all_nodes_prereqs.filter(|(n, prereqs)| visited.is_superset(prereqs) && !visited.contains(n)).map(|(n, _)| n).collect();

    // Reverse sort so that we can pop later
    satisfied_nodes.sort_unstable_by(|a, b| b.cmp(a));
    satisfied_nodes
}
