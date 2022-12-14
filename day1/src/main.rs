use std::env;
use std::fs;

fn riddle_part_one(file_path: &String) -> Vec<u32> {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    let calories_per_elf: Vec<u32> = text
        .split("\n\n")
        .map(|elf_block| {
            elf_block
                .split("\n")
                .map(|calorie_str| {
                    let sanitized = calorie_str.trim();
                    if sanitized.len() == 0 {
                        0
                    } else {
                        sanitized
                            .parse::<u32>()
                            .expect("Error parsing caloric count")
                    }
                })
                .sum::<u32>()
        })
        .collect();

    let highest_caloric_count = calories_per_elf.iter().max_by_key(|p| p.clone()).unwrap();
    println!("Highest caloric count: {}", highest_caloric_count);
    calories_per_elf
}

fn riddle_part_two(file_path: &String) {
    let mut calories_per_elf = riddle_part_one(file_path);
    calories_per_elf.sort_by(|a, b| b.cmp(a));

    let top_three_total_cal = calories_per_elf[0..3].iter().sum::<u32>();
    println!("Top three elfes are carying: {:?}", top_three_total_cal);
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
