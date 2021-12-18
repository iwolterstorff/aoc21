use std::{path::Path, str::FromStr};

use anyhow::{Error, Result};
use aoc21::tokenize_file;

enum Movement {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Movement {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        let distance: i32 = split[1].parse().expect("Couldn't parse to number");
        match split[0] {
            "forward" => Ok(Movement::Forward(distance)),
            "down" => Ok(Movement::Down(distance)),
            "up" => Ok(Movement::Up(distance)),
            _ => Err(anyhow::Error::msg("Invalid direction")),
        }
    }
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn calc_final_position(movements: Vec<Movement>) -> Position {
    let starting = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    movements
        .iter()
        .fold(starting, |position, movement| match movement {
            Movement::Forward(n) => Position {
                horizontal: position.horizontal + n,
                ..position
            },
            Movement::Down(n) => Position {
                depth: position.depth + n,
                ..position
            },
            Movement::Up(n) => Position {
                depth: position.depth - n,
                ..position
            },
        })
}

fn calc_final_position_with_aim(movements: Vec<Movement>) -> Position {
    let starting = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    movements
        .iter()
        .fold(starting, |position, movement| match movement {
            Movement::Forward(n) => Position {
                horizontal: position.horizontal + n,
                depth: position.depth + (position.aim * n),
                ..position
            },
            Movement::Up(n) => Position {
                aim: position.aim - n,
                ..position
            },
            Movement::Down(n) => Position {
                aim: position.aim + n,
                ..position
            },
        })
}

fn get_input(path: &Path) -> Vec<Movement> {
    let movement_strings: Vec<String> = tokenize_file(path).expect("Couldn't tokenize input");
    let movements: Result<Vec<Movement>> = movement_strings
        .iter()
        .map(|s| Movement::from_str(s))
        .collect();
    let movements = movements.expect("Can't cast to Movement structs");
    movements
}

pub fn day_2_1(path: &Path) -> i32 {
    let movements = get_input(path);
    let final_position = calc_final_position(movements);
    (final_position.horizontal * final_position.depth).into()
}

pub fn day_2_2(path: &Path) -> i32 {
    let movements = get_input(path);
    let final_position = calc_final_position_with_aim(movements);
    final_position.horizontal * final_position.depth
}
