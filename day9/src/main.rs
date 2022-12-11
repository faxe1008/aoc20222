use std::env;
use std::fs;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

 fn simulate_rope_with_length(file_path: &String, rope_length:  usize)
{
    let file = File::open(file_path).expect("Error opening file");

    let mut tail_visited_points :Vec<Point>= Vec::new();
    
    let mut rope_segments : Vec<Point> = vec![Point{x: 0, y: 0}; rope_length];
 

    tail_visited_points.push(rope_segments[0].clone());

    let reader = BufReader::new(file);
    for l in reader.lines().into_iter() {
        let line = l.unwrap();

        let direction = match line.chars().nth(0).unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => {panic!("Unknown code");}
        };
        let direction_length = line[2..].parse::<isize>().unwrap();

        println!("{:?} len: {}", direction, direction_length);
        for _ in 0..direction_length {

            // Move the head in the given direction
            let mut head = rope_segments.get_mut(0).unwrap();
            match direction {
                Direction::Down => {
                    head.y += 1;
                },
                Direction::Up => {
                    head.y -= 1;
                },
                Direction::Left => {
                    head.x -= 1;
                },
                Direction::Right => {
                    head.x += 1;
                }
            }

            // iterate over all the other segments
            for rope_segment_index in 1..rope_length {
                let previous_segment = rope_segments[rope_segment_index-1].clone();
                let mut current_segment = rope_segments.get_mut(rope_segment_index).unwrap();

                let segment_distance = (previous_segment.x - current_segment.x, previous_segment.y - current_segment.y);

                if segment_distance.0.abs() > 1 || segment_distance.1.abs() > 1 {

                    let x_movement = segment_distance.0.clamp(-1, 1);
                    let y_movement = segment_distance.1.clamp(-1, 1);

                    current_segment.x += x_movement;
                    current_segment.y += y_movement;
                }
            }
            tail_visited_points.push(rope_segments.last().unwrap().clone());
        }


    }

    tail_visited_points.sort();
    tail_visited_points.dedup();

    println!("Visited Points len: {}", tail_visited_points.len());
}


fn riddle_part_one(file_path: &String)  {
    simulate_rope_with_length(file_path, 2);
}

fn riddle_part_two(file_path: &String) {
    simulate_rope_with_length(file_path, 10);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Provide the input text file!");
    }
    let riddle_num: u32 = args
        .get(1)
        .unwrap()
        .parse()
        .expect("Error parsing riddle num");

    match riddle_num {
        1 => {
            riddle_part_one(args.get(2).unwrap());
        }
        2 => {
            riddle_part_two(args.get(2).unwrap());
        }
        _ => {
            panic!("Unknown riddle part number");
        }
    };
}
