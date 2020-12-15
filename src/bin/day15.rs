use std::collections::HashMap;

type Map = HashMap<u64, u64>;

struct GameIter {
    input: Vec<u64>,
    pos: u64,
    next_item: u64,
    last_occurrences: Map,
}

impl GameIter {
    fn new(starting_items: Vec<u64>) -> GameIter {
        GameIter {
            input: starting_items,
            pos: 0,
            next_item: 0,
            last_occurrences: Map::new(),
        }
    }
}

impl Iterator for GameIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let result;
        if (self.pos as usize) < self.input.len() {
            result = self.input[self.pos as usize];
        } else {
            result = self.next_item;
        }

        if let Some(prev_pos) = self.last_occurrences.get(&result) {
            self.next_item = self.pos - prev_pos;
        } else {
            self.next_item = 0;
        }
        self.last_occurrences.insert(result, self.pos);
        self.pos += 1;
        Some(result)
    }
}

fn nth_element_game_fast(starting_items: &[u32], n: usize) -> u32 {
    let mut buffer = vec![std::u32::MAX; n];
    let target = n - 1;
    for i in 0..starting_items.len() - 1 {
        let num = starting_items[i];
        if i == target {
            return num;
        }
        buffer[num as usize] = i as u32;
    }

    let mut prev = *starting_items.last().unwrap() as usize;
    for i in starting_items.len()-1..n-1 {
        let prev_turn = buffer[prev];
        buffer[prev] = i as u32;
        prev = if prev_turn == std::u32::MAX { 0 } else { i - prev_turn as usize};
    }
    prev as u32
}

fn main() {
    if let Some(value) = GameIter::new(vec![2, 0, 6, 12, 1, 3]).nth(2019) {
        println!("Part 1: {}", value);
    } else {
        println!("Part 1: no result");
    }

    println!(
        "Part 2 fast: {}",
        nth_element_game_fast(&[2, 0, 6, 12, 1, 3], 30_000_000)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(GameIter::new(vec![0, 3, 6]).nth(2019), Some(436));
    }
    #[test]
    fn example_2() {
        assert_eq!(GameIter::new(vec![2, 1, 3]).nth(2019), Some(10));
    }
    #[test]
    fn example_3() {
        assert_eq!(GameIter::new(vec![1, 2, 3]).nth(2019), Some(27));
    }
    #[test]
    fn example_4() {
        assert_eq!(GameIter::new(vec![2, 3, 1]).nth(2019), Some(78));
    }
    #[test]
    fn example_5() {
        assert_eq!(GameIter::new(vec![3, 2, 1]).nth(2019), Some(438));
    }
    #[test]
    fn example_6() {
        assert_eq!(GameIter::new(vec![3, 1, 2]).nth(2019), Some(1836));
    }
}
