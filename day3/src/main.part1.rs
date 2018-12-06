extern crate binary_heap_plus;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::vec::Vec;
use binary_heap_plus::*;

const INPUT_FILENAME: &str = "input.txt";
/*
    Problem: Find the total square area of overalapping rectangles

    - How do we check that two rectangles overlap, and if so, return their
      overlapping area?
      --> Look at their 1D intersections (solve as a 1D line overlap problem x 2)
    
    - Input format: #123 @ 3,2: 5x4 ==> Claim 123, 3 units from left, 2 units from top, 5 wide, 4 tall

    - Conventions for this program: x axis ---->, y axis vertical

*/

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32
}

// Our default ordering is going to be via our bottom right coordinate
// This is so our priority queue can be sorted by the next rectangle to remove
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Rect {
    botright: Point,
    topleft: Point,
    claim: i32,
}

impl std::fmt::Debug for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<({},{})-({},{})>", self.topleft.x, self.topleft.y, self.botright.x, self.botright.y/*, self.botright.x - self.topleft.x, self.botright.y - self.topleft.y*/)
    }
}

/*

    Algorithm for day3 (Algorithm 3.1: Line Sweep)
    - For x coordinate in (0 ... highest x coordinate of set of fabrics)
        - Find the set of rectangles that are within this x-coordinate (Opt 3.1)
        - The problem can now be converted into a 1D line-overlap problem on the y-axis
            - Find the line segments in the y-axis that overlap for 2 or more times (Algorithm 3.2)
            - Since we are adjusting the line sweep by 1 unit each time, add (total overlap distance * 1) to the total area that contributes to the overlap 
    
    Algorithm 3.2: 1D line segment overlap
        - We now have a list of rectangles that are in this x-coordinate
        - We must compute the 1D overlapping segment distance of this cross-section of the rectangles
            - Imagine the rectangular cross-sections at this x-coord (easier to draw), e.g.
                --- 
                 ----
                 --
                ------
                        ----   
                           ---
            - We basically do a 1D line-sweep across this region to find the total intersection area
            - Opt 3.2 applies
            

    Optimizations
        - Opt 3.1: The set of rectangles that are within the sweep line for that x-coordinate can be computed cheaply by considering that
            - We add a rectangle into the set iff its topleft x-coordinate matches the sweep x coord
            - We remove a rectangle from the set iff its botright x-coordinate matches the sweep x coord
            ==> We need a list of rectangles sorted by starting (topleft) x-coordinate
            ==> Our current set of rectangles can be modeled as a priority queue sorted by ending (botright) x-coordinate: we keep peeking at this list on every iteration to see if we need to remove any rectangles
        - Opt 3.2: If we did not remove or add any rectangles to the set of overlapping rectangles in this iteration: we do not need to recompute the result of Algorithm 3.2 since the overlapping distance is still the same

*/

fn main() -> Result<()> {
    println!("Reading AOC Day 3 Input file: input.txt");

    // include_str! is also viable here, but wanted to learn File operations explicitly
    let f = File::open(INPUT_FILENAME)?;

    let mut rects: Vec<Rect> = Vec::new();

    let mut highest_x_coord = 0;
    // Inserting all rectangles into sorted order
    for line in BufReader::new(f).lines() {
        let line_parse = line.unwrap();
        let new_rect = line_to_rect(&line_parse);
        // The highest_x_coord will be the further we will do our line scan
        highest_x_coord = std::cmp::max(highest_x_coord, new_rect.botright.x);
        rects.push(new_rect);
    }

    // Sort the rectangles (custom) by their topleft coordinate - we can iterate through this to
    // find our rectangles for the sweep x coordinate
    rects.sort_by(|r1, r2| r1.topleft.x.cmp(&r2.topleft.x));


    println!("Rects sorted (Vector - Min Topleft): \n\n {:?} \n\n", rects);

    // A minimum heap (sorted by bottom right coordinate (and then by x coordinate first))
    let mut priority_rects: BinaryHeap<&Rect, MinComparator> = BinaryHeap::new_min();


    // Small optimization, don't start from x = 0, but start from the lowest available x coord of rects
    let lowest_x_coord = rects[0].topleft.x;
    let mut rect_idx = 0; // index into the rects vector to get the next vector to 
    let mut overlap_area = 0;
    // Iterate through all x coordinates across fabric
    for sweep_x in lowest_x_coord..(highest_x_coord+1) {
        println!{"Sweep Line X Coord: {}", sweep_x}
        pop_processed_rects(&mut priority_rects, sweep_x);
        push_rects_on_sweep_line(&mut priority_rects, sweep_x, &rects, &mut rect_idx);
        let overlap_distance = calculate_overlap_distance(priority_rects.clone());
        overlap_area += overlap_distance;
        // We now have a binary heap that contains all of the rectangles in this sweep line
        // for this iteration
        // Run Algorithm 3.2 to get the 1D line overlap
        
        // Remember that it's fine if this doesn't print out a total order: a heap is not completely ordered
        println!("Priority Q status: {:?}\n", priority_rects);
    }
    println!("Total Overlap Area: {}", overlap_area);
    Ok(())
}

// Remove all rectangles from prioritiy queue that are exceeding the sweep line's x value
fn pop_processed_rects(priority_rects: &mut BinaryHeap<&Rect, MinComparator>, sweep_x: i32) {
    // TODO
    loop {
        let mut pop_priority_rects = false;

        // Peek at priority queue to see if we need to dequeue a rect
        if let Some(rect) = priority_rects.peek() {
            if rect.botright.x == sweep_x {
                // Rectangle is going out of the sweep line - remove it
                pop_priority_rects = true;
            } else {
                // Closest rect is not in the sweep line, carry on trying to add new rects
                break;
            }
        } 

        // Marked for popping (we did this to avoid mutable/immutable borrow collision above)
        if pop_priority_rects {
            priority_rects.pop();
            continue;
        }
        break
        // If None, break
    }

    println!("Done popping");
}

// Add all of the rectangles in this x-coord to the priority queue
fn push_rects_on_sweep_line<'a>(priority_rects: &mut BinaryHeap<&'a Rect, MinComparator>, sweep_x: i32, rects: &'a Vec<Rect>, rect_idx: &mut usize) {
        loop {
            if *rect_idx == rects.len() { break; }
            if rects[*rect_idx].topleft.x == sweep_x {
                priority_rects.push(&rects[*rect_idx]);
                *rect_idx += 1;
            } else {
                break;
            }
        }

        println!("Done pushing");
}

// Given the current list of rectangles in the sweep line, calculate the line overlap
fn calculate_overlap_distance(priority_rects: BinaryHeap<&Rect, MinComparator>) -> u32 {
    // No rects == no overlap
    if priority_rects.len() == 0 { return 0; }

    // Sort vectors by their starting y-coordinate
    let mut rects: Vec<&Rect> = priority_rects.into_vec();
    rects.sort_by(|r1, r2| r1.topleft.y.cmp(&r2.topleft.y));

    // We also sort them by their ending y-coordinate to decrement as ncessary
    let mut rects_end: Vec<&Rect> = rects.clone();
    rects_end.sort_by(|r1, r2| r1.botright.y.cmp(&r2.botright.y));
    
    let (mut rects_idx, mut rects_end_idx) = (0, 0);
    let mut current_num_overlap = 0; 
    let mut total_intersection_distance = 0;

    let lowest_y = rects.first().unwrap().topleft.y;
    let highest_y = rects_end.last().unwrap().botright.y;
    // Now: run a 1D line-scan in the y-axis across these rectangles
    // Every time we encounter a rectangle, we increment a counter
    // Every time we go beyond the bounds of a rectangle, we decrement it
    // If the counter is >= 2, we have an intersection area: count it
    for sweep_y in lowest_y..(highest_y + 1) {
        // println!("Checking sweep_y = {}", sweep_y);


        // Remove rects past the sweep line
        loop {
            if rects_end_idx == rects_end.len() { break; }

            if sweep_y == rects_end[rects_end_idx].botright.y {
                current_num_overlap -= 1;
                rects_end_idx += 1;
            } else {  break; }
        }

        // Add rects that are in the sweep line
        loop {
            if rects_idx == rects.len() { break; }

            if sweep_y == rects[rects_idx].topleft.y {
                current_num_overlap += 1;
                rects_idx += 1;
            } else { break; }
        }

        // After the adjustments: add this y coord to the intersection sum if 
        // we have >= 2 intersections here
        if current_num_overlap >= 2 {
             total_intersection_distance += 1; 
             // println!("Increased overlap at sweep_y: {}", sweep_y);
        }
    }
    // Return the intersected distance we've accumulated
    total_intersection_distance
}

fn line_to_rect(line: &str) -> Rect {
    // Input format: #123 @ 3,2: 5x4 
    //              ==> Claim 123, 3 units from left, 2 units from top, 5 wide, 4 tall
    let at_index = line.find('@').unwrap();
    let comma_index = line.find(',').unwrap();
    let colon_index = line.find(':').unwrap();
    let x_index = line.find('x').unwrap();
    let claim: i32 = line[1..(at_index - 1)].parse().unwrap();
    let top_x: i32 = line[(at_index + 2)..comma_index].parse().unwrap();
    let top_y: i32 = line[(comma_index + 1)..colon_index].parse().unwrap();
    let bot_x: i32 = top_x + line[(colon_index + 2)..x_index].parse::<i32>().unwrap();
    let bot_y: i32 = top_y + line[(x_index + 1)..].parse::<i32>().unwrap();
    Rect { claim: claim, topleft: Point{x: top_x, y: top_y}, botright: Point{x: bot_x, y: bot_y}}
}