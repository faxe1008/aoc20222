use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_char_value(chr: char) -> u64 {
    if chr >= 'a' && chr <= 'z' {
        (chr as u8 - 'a' as u8) as u64 + 1
    } else if chr >= 'A' && chr <= 'Z' {
        (chr as u8 - 'A' as u8) as u64 + 27
    } else {
        panic!("WTF");
    }
}

fn riddle_part_one(file_path: &String) {
    let file = File::open(file_path).expect("Error opening file");
    let reader = BufReader::new(file);

    let mut total_value_counter: u64 = 0;

    for l in reader.lines().into_iter() {
        let ll = l.unwrap();
        let line = ll.trim();

        let first_compartment: Vec<char> = line[0..(line.len() / 2)].chars().collect();
        let second_compartment: Vec<char> = line[(line.len() / 2)..line.len()].chars().collect();

        let mut matchingChars: Vec<char> = first_compartment
            .iter()
            .filter(|item| second_compartment.contains(item))
            .map(|&x| x)
            .collect();
        matchingChars.sort();
        matchingChars.dedup();

        if matchingChars.len() != 1 {
            panic!("WTF");
        }

        total_value_counter += get_char_value(*matchingChars.get(0).unwrap());
    }
    println!("Total object values: {}", total_value_counter);
}

fn riddle_part_two(file_path: &String) {}

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
