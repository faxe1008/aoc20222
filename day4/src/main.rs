use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Range, RangeInclusive};
use std::str::FromStr;


trait FromStrReprExt {
    fn fromStr(value: &str) -> Result<Self, &'static str> where Self: Sized;
}

impl<T> FromStrReprExt for RangeInclusive<T> 
where T :std::str::FromStr, <T as FromStr>::Err: std::fmt::Debug {
    fn fromStr(value: &str) -> Result<Self, &'static str> {
        let range_components : Vec<&str> = value.split('-').collect();
        if range_components.len() != 2 {
            return Err("Unknown range format");
        }
        let start = range_components[0].parse::<T>();
        let end = range_components[1].parse::<T>();
    
        if start.is_err() || end.is_err() {
            return Err("Invalid range start/end");
        }
        Ok(start.unwrap()..=end.unwrap())

    }
}


trait ContainsExt {
    fn encloses(&self, other: &Self) -> bool;
    fn encloses_partially(&self, other: &Self) -> bool;
}

impl<T: std::cmp::PartialOrd> ContainsExt for RangeInclusive<T> {
    fn encloses(&self, other: &Self) -> bool {
        self.start() >= other.start() && self.end() <= other.end()
    }
    fn encloses_partially(&self, other: &Self) -> bool 
    {
        self.encloses(other) || 
        (self.start() >= other.start() && self.start() <= other.end()) ||
        (self.end() >= other.start() && self.end() <= other.end())
    }
}


fn riddle_part_one(file_path: &String) {
    let file = File::open(file_path).expect("Error opening file");
    let reader = BufReader::new(file);


    let mut overlapping_range_count : u64 = 0;
    for l in reader.lines().into_iter() {
        let line = l.unwrap();

        let range_list_strings = line.trim().split(',').collect::<Vec<&str>>();
        if range_list_strings.len() != 2 {
            panic!("Invalid format");
        }
        let first_range : RangeInclusive<usize> = RangeInclusive::fromStr(range_list_strings[0]).expect("Error parsing range");
        let second_range : RangeInclusive<usize> = RangeInclusive::fromStr(range_list_strings[1]).expect("Error parsing range");
        if first_range.encloses(&second_range) || second_range.encloses(&first_range){
            overlapping_range_count += 1;
        }
    }
    println!("Number of overlapping ranges: {}", overlapping_range_count);
}

fn riddle_part_two(file_path: &String) {
    let file = File::open(file_path).expect("Error opening file");
    let reader = BufReader::new(file);

    let mut partial_overlap_count : u64 = 0;
    for l in reader.lines().into_iter() {
        let line = l.unwrap();

        let range_list_strings = line.trim().split(',').collect::<Vec<&str>>();
        if range_list_strings.len() != 2 {
            panic!("Invalid format");
        }
        let first_range : RangeInclusive<usize> = RangeInclusive::fromStr(range_list_strings[0]).expect("Error parsing range");
        let second_range : RangeInclusive<usize> = RangeInclusive::fromStr(range_list_strings[1]).expect("Error parsing range");
        if first_range.encloses_partially(&second_range) || second_range.encloses_partially(&first_range){
            partial_overlap_count += 1;
        }
    }
    println!("Number of partially overlapping ranges: {}", partial_overlap_count);
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
