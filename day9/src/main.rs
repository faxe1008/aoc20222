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


fn riddle_part_one(file_path: &String)  {
    let file = File::open(file_path).expect("Error opening file");

    let mut tail_visited_points :Vec<Point>= Vec::new();
    let mut head_position = Point { x: 0, y: 0};
    let mut tail_position = Point { x: 0, y: 0};
    tail_visited_points.push(tail_position.clone());

    let display_pos = |head_pos: &Point, tail_pos: &Point, visited_points: &Vec<Point>|{
        for y in -5..5 {
            for x in -5..5 {
                let was_visited = visited_points.iter().any(|pt| pt.x == x && pt.y == y);

                let char = if x == head_pos.x && y == head_pos.y {
                    'H'
                }else if x == tail_pos.x && y == tail_pos.y {
                    'T'
                } else if was_visited {
                    '#'
                }else {
                    '.'
                };
                print!("{}", char);
            }
            print!("\n");
        }
    };

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
            match direction {
                Direction::Down => {
                    head_position.y += 1;
                },
                Direction::Up => {
                    head_position.y -= 1;
                },
                Direction::Left => {
                    head_position.x -= 1;
                },
                Direction::Right => {
                    head_position.x += 1;
                }
            }

            let head_to_tail_distance = (head_position.x - tail_position.x, head_position.y - tail_position.y);
            if head_to_tail_distance.0.abs() > 1 || head_to_tail_distance.1.abs() > 1 {

                let tail_x_movement = head_to_tail_distance.0.clamp(-1, 1);
                let tail_y_movement = head_to_tail_distance.1.clamp(-1, 1);

                tail_position.x += tail_x_movement;
                tail_position.y += tail_y_movement;
                tail_visited_points.push(tail_position.clone());
            }

            //display_pos(&head_position, &tail_position, &tail_visited_points);
            //println!("=========================================================================")
        }


    }

    tail_visited_points.sort();
    tail_visited_points.dedup();

    println!("Visited Points len: {}", tail_visited_points.len());
    //println!("Visited Points: {:?}", tail_visited_points);

}

fn riddle_part_two(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");

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
