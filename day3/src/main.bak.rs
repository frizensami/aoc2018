use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::vec::Vec;

const INPUT_FILENAME: &str = "input.txt";
/*
    Problem: Find the total square area of overalapping rectangles

    - How do we check that two rectangles overlap, and if so, return their
      overlapping area?
      --> Look at their 1D intersections (solve as a 1D line overlap problem x 2)
    
    - Input format: #123 @ 3,2: 5x4 ==> Claim 123, 3 units from left, 2 units from top, 5 wide, 4 tall

    - Conventions for this program: x axis ---->, y axis vertical

*/

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Rect {
    claim: i32,
    topleft: Point,
    botright: Point
}

fn main() -> Result<()> {
    println!("Reading AOC Day 2 Input file: input.txt");

    // include_str! is also viable here, but wanted to learn File operations explicitly
    let f = File::open(INPUT_FILENAME)?;

    let mut rects: Vec<Rect> = Vec::new();

    // Collecting all strings in the file for futher processing 
    for line in BufReader::new(f).lines() {
        let line_parse = line.unwrap();
        rects.push(line_to_rect(&line_parse))
    }

    // List of all rectangles representing the intersections between input list of rectangles
    let mut all_intersections: Vec<Rect> = Vec::new();
    for (i, rect) in rects.iter().enumerate() {
        let rest = &rects[i+1..];
        for r2 in rest {
            let intersect = rectangular_intersection(rect, r2);
            match intersect {
                None => println!("No intersection between \n{:?} and \n{:?}\n", rect, r2),
                Some(r) => { 
                    println!("Intersection between \n{:?} and \n{:?} = \n{:?}\n", rect, r2, r);
                    all_intersections.push(r);
                }
            }
        }
    }

    // Now we need to find the geometric union of all these rectangles
    

    Ok(())
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

// Amazing viz at https://silentmatt.com/rectangle-intersection/
// Returns the rectangle that represents the intersections between these 2 rectangles
fn rectangular_intersection(r1: &Rect, r2: &Rect) -> Option<Rect> {
    if r1.topleft.x < r2.botright.x && r1.botright.x > r2.topleft.x &&
        r1.topleft.y < r2.botright.y && r1.botright.y > r2.topleft.y {
            // Intersection
            let intersect_top_left_x = std::cmp::max(r1.topleft.x, r2.topleft.x);
            let intersect_top_left_y = std::cmp::max(r1.topleft.y, r2.topleft.y);
            let intersect_bot_right_x = std::cmp::min(r1.botright.x, r2.botright.x);
            let intersect_bot_right_y = std::cmp::min(r1.botright.y, r2.botright.y);
            Some(Rect { claim: -1, topleft: Point{x: intersect_top_left_x, y: intersect_top_left_y}, botright: Point{x: intersect_bot_right_x, y: intersect_bot_right_y}})
        }
        else {
            None
        }
}
