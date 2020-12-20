use anyhow::{anyhow, Result};
use combine::parser::char::{char, digit, letter, newline};
use combine::parser::repeat::{many1, sep_by1, sep_end_by1, skip_many};
use combine::stream::position;
use combine::{
    attempt, between, from_str, not_followed_by, EasyParser, ParseError, Parser, Stream,
};
use gumdrop::Options;
use std::collections::HashMap;

enum Rule {
    Lit(char),
    Ref(Vec<Vec<u32>>),
}

fn rules<Input>() -> impl Parser<Input, Output = HashMap<u32, Rule>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<char, Input::Range, Input::Position>,
{
    let whitespace = || skip_many(char(' ').or(char('\t')));
    let num = || from_str(many1::<String, _, _>(digit()).skip(whitespace()));
    let lit = between(char('"'), char('"'), letter())
        .skip(whitespace())
        .map(Rule::Lit);
    let numlist = many1(num());
    let list = sep_by1(numlist, char('|').skip(whitespace())).map(Rule::Ref);
    let rule = (num().skip((char(':'), whitespace())), lit.or(list));
    sep_by1(rule, attempt((newline(), not_followed_by(newline()))))
}

fn parse(input: &str) -> Option<(HashMap<u32, Rule>, Vec<String>)> {
    let line = many1(letter());
    let lines = sep_end_by1(line, newline());
    let mut input_parser = (rules().skip((newline(), newline())), lines);
    match input_parser.easy_parse(position::Stream::new(&*input)) {
        Ok((val, _)) => Some(val),
        Err(err) => {
            println!("Error while parsing input: {}", err);
            None
        }
    }
}

fn accept<I>(index: &u32, rules: &HashMap<u32, Rule>, mut chars: I) -> Result<Vec<I>>
where
    I: Iterator<Item = char> + Clone,
{
    let rule = rules
        .get(index)
        .ok_or(anyhow!("Rule id not found: {}", index))?;
    let mut result = Vec::new();
    match rule {
        Rule::Lit(val) => {
            if chars.next() == Some(*val) {
                result.push(chars);
            }
        }
        Rule::Ref(disj) => {
            for rulelist in disj {
                let mut iters = vec![chars.clone()];
                for rule in rulelist {
                    let mut new_iters = Vec::new();
                    for iter in iters.into_iter() {
                        let mut partial_result = accept(rule, rules, iter)?;
                        new_iters.append(&mut partial_result);
                    }
                    iters = new_iters;
                }
                result.append(&mut iters);
            }
        }
    }
    Ok(result)
}

fn number_matching_lines(rules: &HashMap<u32, Rule>, lines: &[String]) -> Result<usize> {
    let mut counter = 0;
    for line in lines {
        let result = accept(&0, &rules, line.chars())?;
        if result.into_iter().any(|mut i| i.next().is_none()) {
            counter += 1;
        }
    }
    Ok(counter)
}

#[derive(Debug, Options)]
struct Arguments {
    #[options(free)]
    input_file: String,
}

fn main() -> Result<()> {
    let opts = Arguments::parse_args_default_or_exit();
    let input = std::fs::read_to_string(&opts.input_file)?;
    if let Some((mut rules, input)) = parse(&input) {
        println!("Part 1: {}", number_matching_lines(&rules, &input)?);
        rules.insert(8, Rule::Ref(vec![vec![42], vec![42, 8]]));
        rules.insert(11, Rule::Ref(vec![vec![42, 31], vec![42, 11, 31]]));
        println!("Part 2: {}", number_matching_lines(&rules, &input)?);
    }
    Ok(())
}
