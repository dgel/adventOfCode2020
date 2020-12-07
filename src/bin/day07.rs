use combine::parser::choice::optional;
use combine::parser::repeat::many1;
use combine::stream::position;
use combine::{from_str, EasyParser, Parser};
use combine::{
    parser::char::{char, digit, letter, spaces, string},
    sep_by1,
};

use gumdrop::Options;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;

struct BagSpec {
    number: usize,
    bagtype: String,
}

fn parse_rules(filepath: &str) -> BTreeMap<String, Vec<BagSpec>> {
    let word = || many1::<String, _, _>(letter());
    let num = from_str(many1::<String, _, _>(digit()));
    let bagtype = || (word().skip(spaces()), word()).map(|(w1, w2)| format!("{} {}", w1, w2));
    let bagword = || (string("bag"), optional(char('s')));
    let bagspec = (num.skip(spaces()), bagtype().skip((spaces(), bagword())))
        .map(|(number, bagtype)| BagSpec { number, bagtype });
    let bagspecs = sep_by1::<Vec<BagSpec>, _, _, _>(bagspec, char(',').skip(spaces()));
    let no_bags = string("no other bags").map(|_| Vec::new());

    let lhs = bagtype().skip((spaces(), bagword()));
    let rhs = no_bags.or(bagspecs);
    let rule = || {
        (
            lhs.skip((spaces(), string("contain"), spaces())),
            rhs.skip(char('.')),
        )
    };

    let mut input = String::new();
    let rules = || many1::<BTreeMap<String, Vec<BagSpec>>, _, _>(rule().skip(spaces()));

    match File::open(filepath) {
        Ok(file) => {
            let mut bufreader = std::io::BufReader::new(file);
            if bufreader.read_to_string(&mut input).is_ok() {
                match rules().easy_parse(position::Stream::new(&*input)) {
                    Ok((val, _)) => val,
                    Err(err) => {
                        println!("Parse error: {}", err);
                        BTreeMap::new()
                    }
                }
            } else {
                println!("could not open file '{}'", filepath);
                BTreeMap::new()
            }
        }
        Err(error) => {
            println!("could not open file '{}': {}", filepath, error);
            BTreeMap::new()
        }
    }
}

#[derive(Debug, Options)]
struct Arguments {
    #[options(free)]
    input_file: String,
}

fn num_bags_transitively_contain(
    contained_bag: &str,
    rules: &BTreeMap<String, Vec<BagSpec>>,
) -> usize {
    let mut num_bags = 0;
    let mut cache = BTreeMap::new();

    // recursive function declared inside function to avoid using the same cache
    // with different target bagtype
    fn transitively_contains(
        bag: &str,
        contained_bag: &str,
        rules: &BTreeMap<String, Vec<BagSpec>>,
        cache: &mut BTreeMap<String, bool>,
    ) -> bool {
        if let Some(&result) = cache.get(bag) {
            return result;
        }

        if let Some(specs) = rules.get(bag) {
            for spec in specs {
                if spec.bagtype == contained_bag
                    || transitively_contains(&spec.bagtype, contained_bag, rules, cache)
                {
                    cache.insert(bag.into(), true);
                    return true;
                }
            }
            cache.insert(bag.into(), false);
            return false;
        } else {
            println!("No rule found for bag '{}'", bag);
            return false;
        }
    }

    for bag in rules.keys() {
        if transitively_contains(bag, contained_bag, rules, &mut cache) {
            num_bags += 1;
        }
    }
    num_bags
}

fn number_contained_bags(bag: &str, rules: &BTreeMap<String, Vec<BagSpec>>) -> usize {
    fn number_contained_bags_memoized(
        bag: &str,
        rules: &BTreeMap<String, Vec<BagSpec>>,
        cache: &mut BTreeMap<String, usize>,
    ) -> usize {
        if let Some(&result) = cache.get(bag) {
            return result;
        }

        let mut num_bags = 0;
        if let Some(specs) = rules.get(bag) {
            for spec in specs {
                num_bags += spec.number * (1 + number_contained_bags(&spec.bagtype, rules));
            }
        } else {
            println!("No rule found for bag '{}'", bag);
        }
        cache.insert(bag.into(), num_bags);
        num_bags
    }

    let mut cache = BTreeMap::new();
    return number_contained_bags_memoized(bag, rules, &mut cache);
}

fn main() {
    let opts = Arguments::parse_args_default_or_exit();
    let rules = parse_rules(&opts.input_file);
    if !rules.is_empty() {
        println!(
            "Part 1: {}",
            num_bags_transitively_contain("shiny gold", &rules)
        );
        println!("Part 2: {}", number_contained_bags("shiny gold", &rules));
    }
}
