use anyhow::Result;
use combine::parser::char::{char, digit, newline};
use combine::parser::repeat::{chainl1, many1, sep_end_by1, skip_many};
use combine::stream::position;
use combine::{between, from_str, parser, EasyParser, ParseError, Parser, Stream};
use gumdrop::Options;

enum Operation {
    Sum(Box<Operation>, Box<Operation>),
    Mul(Box<Operation>, Box<Operation>),
    Lit(u64),
}

impl Operation {
    fn evaluate(&self) -> u64 {
        match self {
            Operation::Sum(l, r) => l.evaluate() + r.evaluate(),
            Operation::Mul(l, r) => l.evaluate() * r.evaluate(),
            Operation::Lit(val) => *val,
        }
    }
}

fn whitespace<Input>() -> impl Parser<Input, Output = ()>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<char, Input::Range, Input::Position>,
{
    skip_many(char(' ').or(char('\t')))
}

fn expr_<Input>() -> impl Parser<Input, Output = Box<Operation>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<char, Input::Range, Input::Position>,
{
    let num = || from_str(many1::<String, _, _>(digit()).skip(whitespace()));
    let lit = num().map(|n| Box::new(Operation::Lit(n)));
    let group = between(
        char('(').skip(whitespace()),
        char(')').skip(whitespace()),
        expr(),
    );
    let op = char('+').or(char('*')).skip(whitespace()).map(|c| {
        move |l, r| {
            if c == '+' {
                Box::new(Operation::Sum(l, r))
            } else {
                Box::new(Operation::Mul(l, r))
            }
        }
    });
    chainl1(lit.or(group), op)
}

parser! {
    fn expr[Input]()(Input) -> Box<Operation>
    where [Input: Stream<Token = char>]
    {
        expr_()
    }
}

fn operand<Input>() -> impl Parser<Input, Output = Box<Operation>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<char, Input::Range, Input::Position>,
{
    let num = || from_str(many1::<String, _, _>(digit()).skip(whitespace()));
    let lit = num().map(|n| Box::new(Operation::Lit(n)));
    let group = between(
        char('(').skip(whitespace()),
        char(')').skip(whitespace()),
        expr2(),
    );
    group.or(lit)
}

fn addition_<Input>() -> impl Parser<Input, Output = Box<Operation>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<char, Input::Range, Input::Position>,
{
    let op = char('+')
        .skip(whitespace())
        .map(|_| |l, r| Box::new(Operation::Sum(l, r)));
    chainl1(operand(), op)
}

parser! {
    fn addition[Input]()(Input) -> Box<Operation>
    where [Input: Stream<Token = char>]
    {
        addition_()
    }
}

fn expr2_<Input>() -> impl Parser<Input, Output = Box<Operation>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<char, Input::Range, Input::Position>,
{
    let op = char('*')
        .skip(whitespace())
        .map(|_| |l, r| Box::new(Operation::Mul(l, r)));
    chainl1(addition(), op)
}

parser! {
    fn expr2[Input]()(Input) -> Box<Operation>
    where [Input: Stream<Token = char>]
    {
        expr2_()
    }
}

fn parse(input: &str) -> Vec<Box<Operation>> {
    match sep_end_by1(expr(), newline()).easy_parse(position::Stream::new(&*input)) {
        Ok((val, _)) => val,
        Err(err) => {
            println!("Error while parsing input: {}", err);
            Vec::new()
        }
    }
}

fn parse2(input: &str) -> Vec<Box<Operation>> {
    match sep_end_by1(expr2(), newline()).easy_parse(position::Stream::new(&*input)) {
        Ok((val, _)) => val,
        Err(err) => {
            println!("Error while parsing input: {}", err);
            Vec::new()
        }
    }
}

#[derive(Debug, Options)]
struct Arguments {
    #[options(free)]
    input_file: String,
}

fn main() -> Result<()> {
    let opts = Arguments::parse_args_default_or_exit();
    let input = std::fs::read_to_string(&opts.input_file)?;
    let lines = parse(&input);
    if !lines.is_empty() {
        println!(
            "Part 1: {}",
            lines.iter().map(|l| l.evaluate()).sum::<u64>()
        );
    }
    let lines = parse2(&input);
    if !lines.is_empty() {
        println!(
            "Part 2: {}",
            lines.iter().map(|l| l.evaluate()).sum::<u64>()
        );
    }
    Ok(())
}
