use std::fs::File;
use std::io::prelude::*;

use bitvec::prelude::*;
use gumdrop::Options;

struct Form {
    positive_answers: Vec<u8>,
}

impl Form {
    fn new(positive_answers: Vec<u8>) -> Self {
        Form { positive_answers }
    }
}

fn read_forms(filepath: &str) -> Vec<Vec<Form>> {
    match File::open(filepath) {
        Ok(mut file) => {
            let mut input = String::new();
            if file.read_to_string(&mut input).is_ok() {
                input
                    .split("\n\n")
                    .map(|group| {
                        group
                            .split_whitespace()
                            .map(|line| Form::new(line.trim().bytes().collect()))
                            .collect()
                    })
                    .collect()
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

fn total_positive_answers(forms: &[Form]) -> usize {
    let mut bitvec = bitvec![0; 256];
    for form in forms {
        for &byte in &form.positive_answers {
            bitvec.set(byte as usize, true);
        }
    }
    bitvec.count_ones()
}

fn sum_positive_answers(group_forms: &[Vec<Form>]) -> usize {
    group_forms
        .iter()
        .map(|group| total_positive_answers(&group))
        .sum()
}

fn number_unanimous_answers(forms: &[Form]) -> usize {
    forms
        .iter()
        .map(|form| {
            let mut bitvec = bitvec![0; 256];
            for &byte in &form.positive_answers {
                bitvec.set(byte as usize, true);
            }
            bitvec
        })
        .fold(bitvec![1; 256], |acc, bv| acc & bv)
        .count_ones()
}

fn sum_unanimous_answers(group_forms: &[Vec<Form>]) -> usize {
    group_forms
        .iter()
        .map(|group| number_unanimous_answers(&group))
        .sum()
}

#[derive(Debug, Options)]
struct Arguments {
    #[options(free)]
    input_file: String,
}

fn main() {
    let opts = Arguments::parse_args_default_or_exit();
    let forms = read_forms(&opts.input_file);
    if !forms.is_empty() {
        println!("Part 1: {}", sum_positive_answers(&forms));
        println!("Part 2: {}", sum_unanimous_answers(&forms));
    }
}
