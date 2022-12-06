use std::env;
use std::fs;
use std::collections::HashSet;


fn find_unique_seq(value: &str, length: usize) -> Option<usize> {
    for index in length..value.len() {
        let  header = value[index-length..index].chars().collect::<HashSet<char>>();
        if header.len() == length {
            return Some(index);
        }
    }
    return None;
}

fn riddle_part_one(file_path: &String)  {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    println!("Header ends at: {:?}", find_unique_seq(&text, 4));
}

fn riddle_part_two(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    println!("Header ends at: {:?}", find_unique_seq(&text, 14));
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
