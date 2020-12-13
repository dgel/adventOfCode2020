use nalgebra::base::{Matrix2, Vector2};
use std::fs::File;
use std::io::prelude::*;

use gumdrop::Options;

type Position = Vector2<i64>;
type Translation = Vector2<i64>;
type Direction = Vector2<i64>;
type Rotation = Matrix2<i64>;

fn parse_bytestr(bstr: &[u8]) -> Option<i64> {
    std::str::from_utf8(bstr)
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
}

#[derive(Debug)]
enum Instruction {
    Translate(Translation),
    Rotate(Rotation),
    Move(i64),
}

impl Instruction {
    fn parse(s: &str) -> Option<Instruction> {
        let bytes = s.as_bytes();
        match bytes[0] {
            b'R' => match &bytes[1..] {
                b"90" => Some(Instruction::Rotate(Rotation::new(0, 1, -1, 0))),
                b"180" => Some(Instruction::Rotate(Rotation::new(-1, 0, 0, -1))),
                b"270" => Some(Instruction::Rotate(Rotation::new(0, -1, 1, 0))),
                _ => None,
            },
            b'L' => match &bytes[1..] {
                b"90" => Some(Instruction::Rotate(Rotation::new(0, -1, 1, 0))),
                b"180" => Some(Instruction::Rotate(Rotation::new(-1, 0, 0, -1))),
                b"270" => Some(Instruction::Rotate(Rotation::new(0, 1, -1, 0))),
                _ => None,
            },
            b'N' => {
                parse_bytestr(&bytes[1..]).map(|n| Instruction::Translate(Translation::new(0, n)))
            }
            b'W' => {
                parse_bytestr(&bytes[1..]).map(|n| Instruction::Translate(Translation::new(-n, 0)))
            }
            b'S' => {
                parse_bytestr(&bytes[1..]).map(|n| Instruction::Translate(Translation::new(0, -n)))
            }
            b'E' => {
                parse_bytestr(&bytes[1..]).map(|n| Instruction::Translate(Translation::new(n, 0)))
            }
            b'F' => parse_bytestr(&bytes[1..]).map(|n| Instruction::Move(n)),
            _ => {
                println!("Could not parse line: {}", s);
                None
            }
        }
    }
}

#[derive(Debug)]
struct BoatState {
    pos: Position,
    direction: Direction,
    waypoint: Position,
}

impl BoatState {
    fn apply_part1(self, i: &Instruction) -> Self {
        let mut result = self;
        match i {
            Instruction::Translate(vec) => result.pos += vec,
            Instruction::Rotate(mat) => result.direction = mat * result.direction,
            Instruction::Move(scalar) => result.pos += result.direction * *scalar,
        }
        result
    }

    fn apply_part2(self, i: &Instruction) -> Self {
        let mut result = self;
        match i {
            Instruction::Translate(vec) => result.waypoint += vec,
            Instruction::Rotate(mat) => result.waypoint = mat * result.waypoint,
            Instruction::Move(scalar) => result.pos += result.waypoint * *scalar,
        }
        result
    }
}

fn read_instructions(filepath: &str) -> Vec<Instruction> {
    match File::open(filepath) {
        Ok(file) => std::io::BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| Instruction::parse(&line))
            .collect(),
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
    let instructions = read_instructions(&opts.input_file);
    if !instructions.is_empty() {
        let mut boatstate = BoatState{pos: Position::new(0,0), direction: Direction::new(1, 0), waypoint: Position::new(0,0)};
        for instr in &instructions {
            boatstate = boatstate.apply_part1(instr);
        }
        println!("Part 1: {}", boatstate.pos.abs().sum());
        let mut boatstate = BoatState{pos: Position::new(0,0), direction: Direction::new(1, 0), waypoint: Position::new(10,1)};
        for instr in &instructions {
            boatstate = boatstate.apply_part2(instr);
        }
        println!("Part 1: {}", boatstate.pos.abs().sum());
    }
}
