use std::collections::HashSet;
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

        let first_compartment: HashSet<char> = line[0..(line.len() / 2)].chars().collect();
        let second_compartment: HashSet<char> =
            line[(line.len() / 2)..line.len()].chars().collect();

        let common_item = first_compartment
            .intersection(&second_compartment)
            .nth(0)
            .unwrap();
        total_value_counter += get_char_value(*common_item);
    }
    println!("Total object values: {}", total_value_counter);
}

fn riddle_part_two(file_path: &String) {
    let file = File::open(file_path).expect("Error opening file");
    let reader = BufReader::new(file);

    let mut all_elf_bags = Vec::new();
    for l in reader.lines().into_iter() {
        let bag_content: HashSet<char> = l.unwrap().chars().collect();
        all_elf_bags.push(bag_content);
    }

    let mut score_across_groups: u64 = 0;
    for group_start_index in (0..all_elf_bags.len()).step_by(3) {
        let first_elf_bag = all_elf_bags[group_start_index].clone();
        let second_elf_bag = all_elf_bags[group_start_index + 1].clone();
        let third_elf_bag = all_elf_bags[group_start_index + 2].clone();

        let common_items = first_elf_bag
            .iter()
            .filter(|item| second_elf_bag.contains(item) && third_elf_bag.contains(item))
            .map(|&y| y)
            .nth(0)
            .unwrap();
        score_across_groups += get_char_value(common_items);
    }
    println!("Score across all groups: {}", score_across_groups);
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
