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

#[derive(Debug, Clone, Copy)]
struct WorkerStatus {
    task: Option<char>,
    timeremaining: u32 
}

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

    // Initialize 5 workers that can pick up tasks
    let mut worker_status_xs = [WorkerStatus {task: None, timeremaining: 0}; 5];
    let mut total_time_taken = 0;

    // If there is a free worker, give them a job to do from the list of current satisfied nodes
    loop {
        let mut cur_satisfied_nodes = find_all_prereq_satisfied_alphabetical(&g, &mut visited, &worker_status_xs);
        
        // Nothing left to search and everyone is done with their tasks
        if cur_satisfied_nodes.len() == 0 && find_free_worker(&mut worker_status_xs).is_none() { break; }

        //println!("Satisfied Nodes: {:?}", cur_satisfied_nodes);
        // Either we have nodes left to satisfy, or workers are currently in use
        while find_free_worker(&mut worker_status_xs).is_some() && cur_satisfied_nodes.len() > 0 {
            // Assign work to workers
            // Only do something when we have a free worker and nodes to visit
            // Give available work to workers (aka timeremaining = 0)
            let worker = find_free_worker(&mut worker_status_xs).unwrap();
            let new_task = cur_satisfied_nodes.pop().unwrap();
            worker.task = Some(new_task);
            worker.timeremaining = 61 + ((new_task as u32) - ('A' as u32));

            //println!{"Worker assigned: {:?}", worker};
        }

        //println!("Pre-Step Worker Status: {:?}", worker_status_xs);
        // At least 1 worker is busy
        let (time_taken, completed_tasks) = step_workers(&mut worker_status_xs);
        total_time_taken += time_taken;
        // At least one worker has finished - need to add that to the visited nodes list
        visited.extend(&completed_tasks);

        //println!("Completed tasks: {:?}, \nWorker Status: {:?}\nTotal Time: {}\n", completed_tasks, worker_status_xs, total_time_taken);
        for t in completed_tasks {
            print!("{}", t);
        }


        if visited.len() == g.node_count() { break; }
    }

    println!();
    println!("Total time taken: {}", total_time_taken);

    Ok(())
}

fn line_to_edge(line: &str) -> (char, char) {
    (line.chars().nth(5).unwrap(), line.chars().nth(36).unwrap())
}

// Search all nodes for those that have their parent nodes visited
fn find_all_prereq_satisfied_alphabetical(g: &DiGraphMap<char, i8>, visited: &mut HashSet<char>, workers: &[WorkerStatus]) -> Vec<char> {
    let all_nodes = g.nodes();
    let all_nodes_prereqs = all_nodes.map(|n| (n, g.neighbors_directed(n, petgraph::Direction::Incoming).collect()));
    // All incoming nodes are visited, and no worker is currently working on this task
    let mut satisfied_nodes: Vec<char> = all_nodes_prereqs.filter(|(n, prereqs)| visited.is_superset(prereqs) && !visited.contains(n) && workers.iter().find(|w| w.task == Some(*n)).is_none()).map(|(n, _)| n).collect();

    // Reverse sort so that we can pop later
    satisfied_nodes.sort_unstable_by(|a, b| b.cmp(a));
    satisfied_nodes
}

fn find_free_worker(workers: &mut [WorkerStatus]) -> Option<&mut WorkerStatus> {
    (workers.iter_mut().find(|w| w.task.is_none()))
}

// Complete at least 1 worker's job, return the time it took and the tasks that were completed
fn step_workers(workers: &mut [WorkerStatus]) -> (u32, Vec<char>) {
    // Minumum time required to make 1 worker finish
    let time_to_step = workers.iter().filter(|w| w.task.is_some()).min_by_key(|w| w.timeremaining).unwrap().timeremaining;

    let mut completed_tasks = Vec::new();
    for w in workers.iter_mut() {
        if w.task.is_some() { 
            w.timeremaining = w.timeremaining - time_to_step;
            if w.timeremaining == 0 {
                completed_tasks.push(w.task.unwrap());
                w.task = None;
            }
        }
    }
    (time_to_step, completed_tasks)
}