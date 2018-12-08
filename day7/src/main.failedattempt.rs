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

    // println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

    // Run a toposort so that we can start from the node with 0 incoming edges (no deps)
    let topo_nodes = toposort(&g, None).unwrap();

    let mut visited: HashSet<char> = HashSet::new();
    let mut frontier: BinaryHeap<char, MinComparator> = BinaryHeap::new_min();
    let mut cur_node = *topo_nodes.first().unwrap();

    // Add all neighbors to the frontier: will be sorted by char

    loop {
        print!("{}", cur_node);
        visited.insert(cur_node);
        if visited.len() == g.node_count() { break; }

        // List all neighbors that aren't visited yet and aren't in the frontier
        let mut new_nbrs: Vec<char> = g.neighbors_directed(cur_node, petgraph::Direction::Outgoing).filter(|n| !visited.contains(n) && (frontier.iter().find(|x| *x == n) == None)).collect();
        new_nbrs.sort();

        // Expand sorted frontier to contain only non-visited nodes
        frontier.extend(&new_nbrs);
        println!(" - visited: {:?}, new nbrs {:?}, frontier = {:?}", visited, new_nbrs, frontier);

        // Move on to alphabetically next node in frontier
        // We loop to make sure we are selecting a candidate that has all pre-reqs fulfilled
        let mut not_fulfulled_prereq: Vec<char> = Vec::new();
        loop {
            let candidate = frontier.pop().or_else(|| std::process::exit(1)).unwrap();
            // Our visited set must contain all of these nodes, otherwise, try next node
            if visited.is_superset(&g.neighbors_directed(candidate, petgraph::Direction::Incoming).collect()) {
                frontier.extend(&not_fulfulled_prereq);
                cur_node = candidate;
                break;
            } else {
                println!("Candidate {} is not fulfilled", candidate);
                not_fulfulled_prereq.push(candidate);
            }
        }
    }

    println!();

    /*
    println!("Nodes in topo order");
    for t in topo_nodes {
        print!("{} -> ", t);
    }
*/

    Ok(())
}

fn line_to_edge(line: &str) -> (char, char) {
    (line.chars().nth(5).unwrap(), line.chars().nth(36).unwrap())
}

fn alphabetical_dfs_print(g: &DiGraphMap<char, i8>, cur_node: &char, visited: &mut HashSet<char>) {}
