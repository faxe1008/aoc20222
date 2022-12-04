use core::panic;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
enum RPSPick {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq)]
enum RPSRoundResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl RPSPick {
    // the result from the perspective of other
    fn check(self, other: &RPSPick) -> RPSRoundResult {
        if self == *other {
            return RPSRoundResult::Draw;
        }
        match (self, other) {
            (RPSPick::Scissors, RPSPick::Rock) => RPSRoundResult::Win,
            (RPSPick::Rock, RPSPick::Paper) => RPSRoundResult::Win,
            (RPSPick::Paper, RPSPick::Scissors) => RPSRoundResult::Win,
            (_, _) => RPSRoundResult::Loss,
        }
    }

    fn get_score(self, other: RPSPick) -> u32 {
        let base_score = self.check(&other) as u32;
        let symbol_score = other as u32;
        base_score + symbol_score
    }

    fn get_pick_for_result(self, result: &RPSRoundResult) -> RPSPick {
        match (self, &result) {
            (RPSPick::Rock, RPSRoundResult::Win) => RPSPick::Paper,
            (RPSPick::Rock, RPSRoundResult::Loss) => RPSPick::Scissors,

            (RPSPick::Paper, RPSRoundResult::Win) => RPSPick::Scissors,
            (RPSPick::Paper, RPSRoundResult::Loss) => RPSPick::Rock,

            (RPSPick::Scissors, RPSRoundResult::Win) => RPSPick::Rock,
            (RPSPick::Scissors, RPSRoundResult::Loss) => RPSPick::Paper,
            (a, RPSRoundResult::Draw) => a,
        }
    }

    fn get_score_for_desired_result(self, result: RPSRoundResult) -> u32 {
        let symbol_score = self.get_pick_for_result(&result) as u32;
        let base_score = result as u32;
        base_score + symbol_score
    }
}

fn riddle_part_one(file_path: &String) {
    let file = File::open(file_path).expect("Error opening file");
    let reader = BufReader::new(file);

    let mut score: u32 = 0;
    for line in reader.lines().into_iter() {
        let contents = line.unwrap();
        if contents.len() < 3 {
            continue;
        }
        let opponent_pick = match contents.chars().nth(0).expect("Unknown symbol") {
            'A' => RPSPick::Rock,
            'B' => RPSPick::Paper,
            'C' => RPSPick::Scissors,
            _ => {
                panic!("Unknown char");
            }
        };

        let my_pick = match contents.chars().nth(2).expect("Unknown symbol") {
            'X' => RPSPick::Rock,
            'Y' => RPSPick::Paper,
            'Z' => RPSPick::Scissors,
            _ => {
                panic!("Unknown char");
            }
        };

        score += opponent_pick.get_score(my_pick);
    }
    println!("Final score: {:?}", score);
}

fn riddle_part_two(file_path: &String) {
    let file = File::open(file_path).expect("Error opening file");
    let reader = BufReader::new(file);

    let mut score: u32 = 0;
    for line in reader.lines().into_iter() {
        let contents = line.unwrap();
        if contents.len() < 3 {
            continue;
        }
        let opponent_pick = match contents.chars().nth(0).expect("Unknown symbol") {
            'A' => RPSPick::Rock,
            'B' => RPSPick::Paper,
            'C' => RPSPick::Scissors,
            _ => {
                panic!("Unknown char");
            }
        };

        let desired_result = match contents.chars().nth(2).expect("Unknown symbol") {
            'X' => RPSRoundResult::Loss,
            'Y' => RPSRoundResult::Draw,
            'Z' => RPSRoundResult::Win,
            _ => {
                panic!("Unknown char");
            }
        };
        score += opponent_pick.get_score_for_desired_result(desired_result);
    }
    println!("Final score: {:?}", score);
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
