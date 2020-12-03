use combine::parser::char::{char, digit, letter, spaces};
use combine::parser::repeat::many1;
use combine::stream::position;
use combine::{from_str, EasyParser, Parser};
use gumdrop::Options;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;

struct PasswordEntry {
    lower_bound: usize,
    upper_bound: usize,
    character: char,
    password: String,
}

fn parse(filepath: &str) -> Vec<PasswordEntry> {
    let num_usize = || from_str(many1::<String, _, _>(digit()));
    let entry = (
        num_usize().skip(char('-')),
        num_usize().skip(spaces()),
        letter().skip(char(':')).skip(spaces()),
        many1(letter()).skip(spaces()),
    )
        .map(
            |(lower_bound, upper_bound, character, password)| PasswordEntry {
                lower_bound,
                upper_bound,
                character,
                password,
            },
        );

    let mut input = String::new();
    let mut entries = many1(entry);

    match File::open(filepath) {
        Ok(file) => {
            let mut bufreader = std::io::BufReader::new(file);
            if bufreader.read_to_string(&mut input).is_ok() {
                match entries.easy_parse(position::Stream::new(&*input)) {
                    Ok((val, _)) => val,
                    Err(err) => {
                        println!("Parse error: {}", err);
                        Vec::new()
                    }
                }
            } else {
                println!("could not open file '{}'", filepath);
                Vec::new()
            }
        }
        Err(error) => {
            println!("could not open file '{}': {}", filepath, error);
            Vec::new()
        }
    }
}

fn password_is_valid(entry: &PasswordEntry) -> bool {
    let count = entry
        .password
        .chars()
        .filter(|&c| c == entry.character)
        .count();
    count >= entry.lower_bound && count <= entry.upper_bound
}

fn num_passwords_valid(entries: &[PasswordEntry]) -> usize {
    entries.iter().filter(|e| password_is_valid(e)).count()
}

fn password_is_valid_otcas(entry: &PasswordEntry) -> bool {
    if entry.lower_bound <= entry.password.len().try_into().unwrap()
        && entry.upper_bound <= entry.password.len()
    {
        entry
            .password
            .chars()
            .zip(1..)
            .filter(|&(c, i)| {
                (i == entry.lower_bound || i == entry.upper_bound) && c == entry.character
            })
            .count()
            == 1
    } else {
        false
    }
}

fn num_passwords_valid_otcas(entries: &[PasswordEntry]) -> usize {
    entries.iter().filter(|e| password_is_valid_otcas(e)).count()
}

#[derive(Debug, Options)]
struct Arguments {
    #[options(free)]
    input_file: String,
}

fn main() {
    let opts = Arguments::parse_args_default_or_exit();
    let entries = parse(&opts.input_file);
    if !entries.is_empty() {
        println!("Part 1: {}", num_passwords_valid(&entries));
        println!("Part 2: {}", num_passwords_valid_otcas(&entries));
    }
}
