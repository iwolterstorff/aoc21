use std::hash::Hash;
use std::path::Path;

use itertools::Itertools;

use aoc21::tokenize_file;

fn get_most_common<T: Eq + Hash + Copy>(input: Vec<T>) -> T {
    let counts = input.iter().counts();
    let result = counts.iter().max_by_key(|x| x.1);
    if let Some(x) = result {
        return **(x.0);
    } else {
        panic!("bad");
    }
}

fn nth_char_in_strings(strings: Vec<&str>, n: usize) -> Vec<char> {
    strings
        .iter()
        .map(|str| str.as_bytes()[n] as char)
        .collect()
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

fn oxygen_co2_product(input: Vec<String>) -> i32 {
    if input.is_empty() {
        panic!("You're dumb");
    }
    let length = input[0].len();
    let mut most_common_at_index: Vec<char> = Vec::with_capacity(input.len());
    let str_input: Vec<&str> = input.iter().map(String::as_str).collect();
    for index in 0..length {
        let bits_at_index = nth_char_in_strings(str_input.clone(), index);
        let most_common = get_most_common(bits_at_index);
        most_common_at_index.push(most_common);
    }

    let oxygen = find_key_value(input.clone(), most_common_at_index.clone(), false);
    let co2 = find_key_value(input.clone(), most_common_at_index.clone(), true);

    oxygen * co2
}

fn find_key_value(mut input: Vec<String>, mut most_common: Vec<char>, find_co2: bool) -> i32 {
    if find_co2 {
        most_common = most_common
            .iter()
            .map(|c| match c {
                '1' => '0',
                '0' => '1',
                _ => c.clone(),
            })
            .collect_vec();
    }

    let mut index = 0;
    while input.len() > 1 {
        input = input
            .iter()
            .filter(|s| {
                let string_bytes = &s.as_bytes();

                string_bytes[index] as char == most_common[index]
            })
            .map(|s| s.clone())
            .collect();

        index += 1;
    }

    i32::from_str_radix(&input[0], 2).unwrap()
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
    fn test_oxygen_co2_product() {
        assert_eq!(
            oxygen_co2_product(EXAMPLE.iter().map(|&x| x.into()).collect()),
            230
        );
    }

    #[test]
    fn test_get_most_common() {
        let input = vec!["hi", "bye", "hello", "hi"];
        assert_eq!(get_most_common(input), "hi");
    }

    #[test]
    fn test_nth_char_in_strings() {
        let input = vec!["the", "quick", "brown", "fox"];
        assert_eq!(nth_char_in_strings(input, 2), vec!['e', 'i', 'o', 'x']);
    }
}
