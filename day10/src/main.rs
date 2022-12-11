use std::env;
use std::fs;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};


trait InstructionTrait {
    fn cycle_count(&self) -> usize; 
}

#[derive(Debug, Clone ,Copy)]
enum Instruction {
    NOOP,
    ADDX(isize)
}
impl InstructionTrait for Instruction {
    fn cycle_count(&self) -> usize {
        match self {
            Instruction::NOOP => 1,
            Instruction::ADDX(_) => 2
        }
    }
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>
}

impl Program {
    fn from_text(text: &str) -> Self {
        let lines : Vec<&str> = text.split('\n').collect();
        let mut instructions = Vec::new();
        for line in lines {
            if line.starts_with("noop") {
                instructions.push(Instruction::NOOP);
            }

            if line.starts_with("addx") {
                let amount = line[5..].parse::<isize>().unwrap();
                instructions.push(Instruction::ADDX(amount));
            }

        }
        Self { instructions}
    }
}

struct CPU {
    x_register: isize,
    current_cycle: usize,
    program: Program,

    ip: usize,
    cycles_remaining_for_instruction: usize
}

impl CPU {
    fn new(program: Program) -> Self {
        let initial_cycle_count = program.instructions[0].cycle_count();
        CPU { x_register: 1, current_cycle: 1, program, ip: 0, cycles_remaining_for_instruction: initial_cycle_count}
    }

    fn has_finished_execution(&self) -> bool {
        self.ip >= self.program.instructions.len()
    }

    fn execute(&mut self, ins: &Instruction) {
        match ins {
            Instruction::NOOP => {},
            Instruction::ADDX(amount) => {self.x_register += amount; }
        }
    }

    fn tick(&mut self){
        if self.cycles_remaining_for_instruction == 1 {
            let current_instruction = self.program.instructions[self.ip];
            self.execute(&current_instruction);
            self.ip += 1;
            if !self.has_finished_execution(){
                self.cycles_remaining_for_instruction = self.program.instructions[self.ip].cycle_count();
            }
        }   else {
            self.cycles_remaining_for_instruction -= 1;
        }
        self.current_cycle += 1;

    }

}


fn riddle_part_one(file_path: &String)  {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    let program = Program::from_text(&text);
    let mut cpu = CPU::new(program);

    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut signal_sum : isize = 0;
    while !cpu.has_finished_execution() {
        println!("Cycle: {}, IP: {:?}, REG: {}", cpu.current_cycle, cpu.ip, cpu.x_register);
        if interesting_cycles.contains(&cpu.current_cycle) {
            signal_sum += cpu.current_cycle as isize * cpu.x_register;
        }
        cpu.tick();

    }
    println!("Signal Sum: {}", signal_sum);
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
