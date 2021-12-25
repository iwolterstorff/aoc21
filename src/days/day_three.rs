use std::hash::Hash;
use std::path::Path;

use itertools::Itertools;

use aoc21::tokenize_file;

// I would like this to be generic, but types are hard
fn get_most_common<T: Eq + Hash + Copy>(input: Vec<T>) -> T {
    let counts = input.iter().counts();
    let result = counts.iter().max_by_key(|x| x.1);
    if let Some(x) = result {
        return **(x.0);
    } else {
        panic!("bad");
    }
}

fn gamma_epsilon_product(input: Vec<String>) -> i32 {
    let length = input[0].len();
    let mut result = String::new();
    for index in 0..length {
        let mut collection = Vec::new();
        for st in &input {
            collection.push(st.as_bytes()[index] as char);
        }
        result.push(get_most_common(collection));
    }
    let gamma = i32::from_str_radix(&result, 2).unwrap();

    // We have to mask out the first `length` bits to take the complement for epsilon
    let mask = (1 << length) - 1;
    let epsilon = (!gamma) & mask;
    gamma * epsilon
}

pub fn day_3_1(path: &Path) -> i32 {
    let bin_strings: Vec<String> = tokenize_file(path).expect("Couldn't get input");

    gamma_epsilon_product(bin_strings)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [&str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn test_gamma_epsilon_product() {
        assert_eq!(
            gamma_epsilon_product(EXAMPLE.iter().map(|&x| x.into()).collect()),
            198
        );
    }

    #[test]
    fn test_get_most_common() {
        let input = vec!["hi", "bye", "hello", "hi"];
        assert_eq!(get_most_common(input), "hi");
    }
}
