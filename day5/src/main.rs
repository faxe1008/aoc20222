use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Range, RangeInclusive};
use std::str::FromStr;

#[derive(Debug)]
enum CraneInstruction {
    MoveCrateFromToStack(usize, usize, usize)
}

impl CraneInstruction {
    fn from_str(value: &str) -> Result<Self, &'static str> {
        // move 3 from 5 to 1
        let instruction_parts = value.trim().split_whitespace().collect::<Vec<&str>>();
        if instruction_parts.len() != 6{
            return Err("Invalid format");
        }
        if  instruction_parts[0] != "move" || 
            instruction_parts[2] != "from" ||
            instruction_parts[4] != "to" {
                return Err("Invalid format");
            }

        let count =  instruction_parts[1].parse::<usize>();
        let source = instruction_parts[3].parse::<usize>();
        let target = instruction_parts[5].parse::<usize>();
            if count.is_err() || source.is_err() || target.is_err() {
                return Err("Invalid format");
            }
         Ok(CraneInstruction::MoveCrateFromToStack(count.unwrap(), source.unwrap() - 1, target.unwrap() - 1))
    }
}

type CargoStack = Vec<char>;
struct CargoBay {
    cargo_stacks: Vec<CargoStack>
}

impl CargoBay {
    fn new(cargo_stacks: Vec<CargoStack>) -> Self {
        Self { cargo_stacks}
    }

    fn execute(&mut self, instruction: CraneInstruction) {
        if let CraneInstruction::MoveCrateFromToStack(count, source, target) = instruction {
            for i in 0..count { 
                let source_crate :Option<char> = {
                    let source_stack = self.cargo_stacks.get_mut(source);
                    if source_stack.is_none() {
                         None
                    }else{
                        source_stack.unwrap().pop()
                    }
                };
               
                let target_stack = self.cargo_stacks.get_mut(target);
                if  target_stack.is_some() && source_crate.is_some() {
                        target_stack.unwrap().push(source_crate.unwrap());
                }
            
        }
        }
    }

    fn print_stack_tops(&self) {
        for stack in &self.cargo_stacks {
            print!("{}", stack.last().unwrap_or(&'_'));
        }
        print!("\n");
    }



}

trait FromStringReprExt {
    fn from_str(value: &str) -> Self;
}

impl FromStringReprExt for CargoStack {
    fn from_str(value: &str) -> Self {
        value.chars().collect()
    }
}



fn riddle_part_one(file_path: &String) {

 /*
    [T]     [Q]             [S]        
    [R]     [M]             [L] [V] [G]
    [D] [V] [V]             [Q] [N] [C]
    [H] [T] [S] [C]         [V] [D] [Z]
    [Q] [J] [D] [M]     [Z] [C] [M] [F]
    [N] [B] [H] [N] [B] [W] [N] [J] [M]
    [P] [G] [R] [Z] [Z] [C] [Z] [G] [P]
    [B] [W] [N] [P] [D] [V] [G] [L] [T]
     1   2   3   4   5   6   7   8   9 
    */

    let mut cargo_bay : CargoBay = CargoBay::new(
        vec![
            CargoStack::from_str("BPNQHDRT"),
            CargoStack::from_str("WGBJTV"),
            CargoStack::from_str("NRHDSVMQ"),
            CargoStack::from_str("PZNMC"),
            CargoStack::from_str("DZB"),
            CargoStack::from_str("VCWZ"),
            CargoStack::from_str("GZNCVQLS"),
            CargoStack::from_str("LGJMDNV"),
            CargoStack::from_str("TPMFZCG")
        ]
    );
   
   
    let file = File::open(file_path).expect("Error opening file");
    let reader = BufReader::new(file);

    for l in reader.lines().into_iter() {
        let instruction_or_error = CraneInstruction::from_str(&l.unwrap());
        if let Ok(instruction) = instruction_or_error{
            println!("Instruction: {:?}", instruction);
            cargo_bay.execute(instruction);
        }
    }


    cargo_bay.print_stack_tops();



}

fn riddle_part_two(file_path: &String) {
   

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
