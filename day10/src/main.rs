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

#[derive(PartialEq, Debug)]
enum InstructionState{
    Executing(usize),
    FetchedNextInstruction(usize),
    Finished
}

struct CPU {
    x_register: isize,
    current_cycle: usize,
    program: Program,

    ip: isize,
    instruction_state: InstructionState
}

impl CPU {
    fn new(program: Program) -> Self {
        let initial_cycle_count = program.instructions[0].cycle_count();
        CPU { x_register: 1, current_cycle: 1, program, ip: 1, instruction_state: InstructionState::Finished}
    }

    fn has_finished_execution(&self) -> bool {
         self.ip  >= self.program.instructions.len() as isize
    }

    fn run_instruction(&mut self, ins: &Instruction) {
        match ins {
            Instruction::NOOP => {},
            Instruction::ADDX(amount) => {self.x_register += amount; }
        }
    }

    fn fetch(&mut self){
        if self.instruction_state  == InstructionState::Finished {
            self.ip += 1;
            self.instruction_state = InstructionState::FetchedNextInstruction(self.program.instructions[self.ip as usize].cycle_count());
        }
        self.current_cycle += 1;

    }

    fn execute(&mut self)  {

        match self.instruction_state {
            InstructionState::Executing(remaining_cycles) => {
                if remaining_cycles > 1 {
                    self.instruction_state = InstructionState::Executing(remaining_cycles - 1);
                } else {
                    let ins = self.program.instructions[self.ip as usize];
                    self.run_instruction(&ins);
                    self.instruction_state = InstructionState::Finished;
                }
            },
            InstructionState::FetchedNextInstruction(cycles) => {
                self.instruction_state = InstructionState::Executing(cycles - 1)
            },
            _ => {}
        }

    }

}


fn riddle_part_one(file_path: &String)  {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    let program = Program::from_text(&text);
    let mut cpu = CPU::new(program);

    while !cpu.has_finished_execution() {
        cpu.fetch();
        println!("Cycle: {}, State: {:?}, IP: {:?}, REG: {}", cpu.current_cycle, cpu.instruction_state, cpu.ip, cpu.x_register);
        cpu.execute();
    }
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
