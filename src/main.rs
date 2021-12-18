mod days;

use std::convert::TryFrom;
use std::path::Path;
use std::str::FromStr;

use anyhow::{Error, Result};
use clap::Parser;

#[derive(Debug)]
struct Day(u16);

impl Day {
    fn new(n: u16) -> Option<Day> {
        if n > 0 && n <= 25 {
            Some(Day(n))
        } else {
            None
        }
    }
}

impl TryFrom<u16> for Day {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> Result<Self> {
        match Day::new(value) {
            Some(d) => Ok(d),
            None => Err(Error::msg("Invalid Day format")),
        }
    }
}

impl FromStr for Day {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number: u16 = s.parse()?;
        Day::try_from(number)
    }
}

#[derive(Debug)]
struct Problem(bool);

impl Problem {
    fn new(b: u16) -> Option<Problem> {
        if b > 0 && b <= 2 {
            Some(Problem((b - 1) != 0))
        } else {
            None
        }
    }
}

impl TryFrom<u16> for Problem {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> Result<Self> {
        match Problem::new(value) {
            Some(p) => Ok(p),
            None => Err(Error::msg("Invalid Problem format")),
        }
    }
}

impl FromStr for Problem {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number: u16 = s.parse()?;
        Problem::try_from(number)
    }
}

#[derive(Parser, Debug)]
#[clap(about, version)]
struct Args {
    day: Day,
    puzzle: Problem,
}

struct Solution {
    input: &'static str,
    function: fn(&Path) -> i32,
}

fn main() {
    let args = Args::parse();
    let soln = get_solution_from_options(args);
    let func = soln.function;
    println!("{}", func(Path::new(soln.input)))
}

fn get_solution_from_options(opts: Args) -> Solution {
    match opts {
        Args {
            day: Day(1),
            puzzle: Problem(false),
        } => Solution {
            input: "inputs/1-1.txt",
            function: days::day_one::day_1_1,
        },
        Args {
            day: Day(1),
            puzzle: Problem(true),
        } => Solution {
            input: "inputs/1-1.txt",
            function: days::day_one::day_1_2,
        },
        Args {
            day: Day(2),
            puzzle: Problem(false),
        } => Solution {
            input: "inputs/2.txt",
            function: days::day_two::day_2_1,
        },
        _ => panic!("You're dumb"),
    }
}
