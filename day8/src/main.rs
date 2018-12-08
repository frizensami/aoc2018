use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::io::prelude::*;
use std::vec::Vec;
use std::slice::Iter;

enum ParseState {
    NumChildNodes,
    NumMetadata,

}

#[derive(Debug)]
struct Node {
    nodeid: u32,
    num_children: u32,
    num_metadata: u32,
    children: Vec<Node>,
    metadata: Vec<u32>
}

const INPUT_FILENAME: &str = "input.txt";
fn main() -> Result<()> {
    println!("Reading AOC Day 8 Input file: input.txt");

    let mut f = File::open(INPUT_FILENAME)?;

    let mut input_string = String::new();
    f.read_to_string(&mut input_string)?;

    let nums: Vec<u32> = input_string.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
    // For each node: we have 4 steps
    // 1. Read # of child nodes
    // 2. Read # of metadata 
    // 3. If any, read child node info
    // 4. If any, read metadata info

    let mut nodeid_cur: u32 = 0;
    let head_node = parse_node(&mut nums.iter(), &mut nodeid_cur);
    println!("Sum metadata: {}", sum_metadata(&head_node));
    println!("Part2 sum: {}", part2_sum(&head_node));

    Ok(())
}

fn parse_node(iter: &mut Iter<u32>, nodeid_cur: &mut u32) -> Node {
    let num_children = iter.next().unwrap();
    let num_metadata = iter.next().unwrap();
    let mut children: Vec<Node> = Vec::new();
    let mut meta: Vec<u32> = Vec::new(); 
    let my_nodeid: u32= *nodeid_cur;
    for _ in 0..*num_children {
        // Iterate through child nodes
        *nodeid_cur += 1;
        children.push(parse_node(iter, nodeid_cur));
    }
    for i in 0..*num_metadata {
        meta.push(*iter.next().unwrap());
    }
    // println!("Node {}- #children: {}, #meta: {}, metadata: {:?}: children: {:?}", my_nodeid, num_children, num_metadata, meta, children);
    Node { nodeid: my_nodeid, num_children: *num_children, num_metadata: *num_metadata, children: children, metadata: meta }
}

fn sum_metadata(node: &Node) -> u32 {
    let local_sum: u32 = node.metadata.iter().sum();
    let mut children_sum = 0;

    for cnode in node.children.iter() { children_sum += sum_metadata(&cnode) }

    local_sum + children_sum
}

/*
If a node has no child nodes, its value is the sum of its metadata entries. So, the value of node B is 10+11+12=33, and the value of node D is 99.

However, if a node does have child nodes, the metadata entries become indexes which refer to those child nodes. A metadata entry of 1 refers to the first child node, 2 to the second, 3 to the third, and so on. The value of this node is the sum of the values of the child nodes referenced by the metadata entries. If a referenced child node does not exist, that reference is skipped. A child node can be referenced multiple time and counts each time it is referenced. A metadata entry of 0 does not refer to any child node.
*/
fn part2_sum(node: &Node) -> u32 {
    if node.num_children == 0 {
        node.metadata.iter().sum()
    } else {
        let mut total_sum: u32 = 0;
        for i in node.metadata.iter() {
            if let Some(n) = node.children.get((i-1) as usize) {
                total_sum += part2_sum(n);
            }

        }
        total_sum
    }
}