use advent_of_code_2020::Mat;
use std::fs::File;
use std::io::prelude::*;

use gumdrop::Options;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

type Map = Mat<Tile>;

impl Tile {
    fn from_char(c: char) -> Option<Tile> {
        match c {
            'L' => Some(Tile::EmptySeat),
            '#' => Some(Tile::OccupiedSeat),
            '.' => Some(Tile::Floor),
            _ => None,
        }
    }
}

fn read_map(filepath: &str) -> Option<Map> {
    match File::open(filepath) {
        Ok(file) => {
            let lines: Option<Vec<Vec<Tile>>> = std::io::BufReader::new(file)
                .lines()
                .map(|line| {
                    line.ok()
                        .map(|s| s.chars().flat_map(|c| Tile::from_char(c)).collect())
                })
                .collect();

            lines.and_then(|v| {
                if !v.is_empty()
                    && !v[0].is_empty()
                    && v.iter().map(|row| row.len()).all(|i| i == v[0].len())
                {
                    let rows = v.len() + 2;
                    let columns = v[0].len() + 2;
                    let mut map = Mat::new(columns, rows, Tile::Floor);
                    for (i, row) in v.iter().enumerate() {
                        for (j, tile) in row.iter().enumerate() {
                            map[(j + 1, i + 1)] = *tile;
                        }
                    }

                    Some(map)
                } else {
                    None
                }
            })
        }
        Err(error) => {
            println!("could not open file '{}': {}", filepath, error);
            None
        }
    }
}

#[derive(Debug, Options)]
struct Arguments {
    #[options(free)]
    input_file: String,
}

fn count_around(map: &Map, row: usize, column: usize, tile: Tile) -> usize {
    let mut result = 0;
    for &(hstep, vstep) in &[
        (1, 1),
        (0, 1),
        (-1, 1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ] {
        let c = (column as isize + hstep) as usize;
        let r = (row as isize + vstep) as usize;
        if map[(c, r)] == tile {
            result += 1;
        }
    }
    result
}

fn count_first_directions(map: &Map, row: usize, column: usize, tile: Tile) -> usize {
    let mut result = 0;
    for &(hstep, vstep) in &[
        (1, 1),
        (0, 1),
        (-1, 1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ] {
        let mut c = column as isize;
        let mut r = row as isize;
        loop {
            c += hstep;
            r += vstep;

            if c < 0 || c >= map.width() as isize || r < 0 || r >= map.height() as isize {
                break;
            }
            let cur_tile = map[(c as usize, r as usize)];
            if cur_tile != Tile::Floor {
                if cur_tile == tile {
                    result += 1;
                }
                break;
            }
        }
    }
    result
}

type CountFun = fn(&Map, usize, usize, Tile) -> usize;

fn step_tile(map: &Map, row: usize, column: usize, occupied_swap_threshold: usize, countfun: CountFun) -> Tile
{
    match map[(column, row)] {
        Tile::Floor => Tile::Floor,
        Tile::EmptySeat => {
            if countfun(map, row, column, Tile::OccupiedSeat) == 0 {
                Tile::OccupiedSeat
            } else {
                Tile::EmptySeat
            }
        }
        Tile::OccupiedSeat => {
            if countfun(map, row, column, Tile::OccupiedSeat) >= occupied_swap_threshold {
                Tile::EmptySeat
            } else {
                Tile::OccupiedSeat
            }
        }
    }
}

fn step_map(map: &Map, occupied_swap_threshold: usize, countfun: CountFun) -> Map 
{
    let mut new_map = Map::new(map.width(), map.height(), Tile::Floor);
    for i in 1..map.width() - 1 {
        for j in 1..map.height() - 1 {
            new_map[(i, j)] = step_tile(map, j, i, occupied_swap_threshold, countfun);
        }
    }
    new_map
}

fn fixed_point(map: &Map, occupied_swap_threshold: usize, countfun: CountFun) -> Map {
    let mut map = map.clone();
    loop {
        let new_map = step_map(&map, occupied_swap_threshold, countfun);
        if new_map == map {
            return map;
        }
        map = new_map;
    }
}


fn main() {
    let opts = Arguments::parse_args_default_or_exit();
    let map_opt = read_map(&opts.input_file);
    if let Some(map) = map_opt {
        println!(
            "Part 1: {}",
            fixed_point(&map, 4, count_around)
                .iter_elements()
                .filter(|&&tile| tile == Tile::OccupiedSeat)
                .count()
        );
        println!(
            "Part 2: {}",
            fixed_point(&map, 5, count_first_directions)
                .iter_elements()
                .filter(|&&tile| tile == Tile::OccupiedSeat)
                .count()
        );
    } else {
        println!("Something went wrong while reading the map");
    }
}
