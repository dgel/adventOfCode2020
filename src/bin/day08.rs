use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;

use bitvec::prelude::*;
use gumdrop::Options;

enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Instruction {
    fn parse(instr: &str) -> Option<Instruction> {
        if instr.is_empty() {
            return None
        }
        let mut part_iter = instr.split_whitespace();
        if let (Some(op_string), Some(offset_string)) = (part_iter.next(), part_iter.next()) {
            if let Ok(offset) = offset_string.trim_start_matches('+').parse() {
                match op_string {
                    "acc" => Some(Instruction::Acc(offset)),
                    "jmp" => Some(Instruction::Jmp(offset)),
                    "nop" => Some(Instruction::Nop(offset)),
                    _ => {
                        println!("Could not match operation '{}'", op_string);
                        None
                    }
                }
            } else {
                println!("Could not parse offset '{}'", offset_string);
                None
            }
        } else {
            println!("Found unexpected number of words on line '{}'", instr);
            None
        }
    }
}

enum ProgramResult {
    Success(isize),
    Repeat(isize),
    Error
}

fn accumulator_before_instruction_repeat(program: &[Instruction]) -> ProgramResult {
    let mut accumulator = 0;
    let mut cur_instruction: isize = 0;
    let mut instruction_executed = bitvec![0; program.len()];
    loop {
        if let Ok(index) = TryInto::<usize>::try_into(cur_instruction) {
            if index >= program.len() {
                return ProgramResult::Success(accumulator);
            }

            if let Some(true) = instruction_executed.get(index) {
                return ProgramResult::Repeat(accumulator);
            }

            match program[index] {
                Instruction::Acc(value) => {
                    accumulator += value;
                    cur_instruction += 1;
                }
                Instruction::Jmp(value) => {
                    cur_instruction += value;
                }
                Instruction::Nop(_) => cur_instruction += 1,
            }
            instruction_executed.set(index, true);
        } else {
            println!(
                "Instruction pointer overflow or underflow: {}",
                cur_instruction
            );
            return ProgramResult::Error;
        }
    }
}

fn find_broken_instruction(program: &mut [Instruction]) -> Option<isize> {
    for i in 0..program.len() {
        match program[i] {
            Instruction::Jmp(value) => {
                program[i] = Instruction::Nop(value);
                if let ProgramResult::Success(value) = accumulator_before_instruction_repeat(program) {
                    return Some(value);
                }
                program[i] = Instruction::Jmp(value);
            }
            Instruction::Nop(value) => {
                program[i] = Instruction::Jmp(value);
                if let ProgramResult::Success(value) = accumulator_before_instruction_repeat(program) {
                    return Some(value);
                }
                program[i] = Instruction::Nop(value);
            }
            _ => {}
        }
    }
    None
}

fn read_program(filepath: &str) -> Vec<Instruction> {
    match File::open(filepath) {
        Ok(mut file) => {
            let mut input = String::new();
            if file.read_to_string(&mut input).is_ok() {
                input.split("\n").filter_map(Instruction::parse).collect()
            } else {
                Vec::new()
            }
        }
        Err(error) => {
            println!("could not open file '{}': {}", filepath, error);
            Vec::new()
        }
    }
}

#[derive(Debug, Options)]
struct Arguments {
    #[options(free)]
    input_file: String,
}

fn main() {
    let opts = Arguments::parse_args_default_or_exit();
    let mut program = read_program(&opts.input_file);
    if !program.is_empty() {
        if let ProgramResult::Repeat(value) = accumulator_before_instruction_repeat(&program) {
            println!("Part 1: {}", value);
        } else {
            println!("Part 1: No result found");
        }
        if let Some(value) = find_broken_instruction(&mut program) {
            println!("Part 2: {}", value);
        } else {
            println!("Part 2: No result found");
        }
    }
}
