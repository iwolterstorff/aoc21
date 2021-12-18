use std::path::Path;

use itertools::Itertools;

use aoc21;

pub fn day_1_1(path: &Path) -> i32 {
    let input = aoc21::tokenize_file_to_integers(path).expect("Couldn't parse integers");
    find_number_of_increases(input)
}

fn find_number_of_increases(input: Vec<i32>) -> i32 {
    input
        .iter()
        .tuple_windows::<(_, _)>()
        .fold(0, |total, (a, b)| if b > a { total + 1 } else { total + 0 })
}

pub fn day_1_2(path: &Path) -> i32 {
    let input = aoc21::tokenize_file_to_integers(path).expect("Couldn't parse integers");
    find_number_of_three_element_increases(input)
}

fn find_number_of_three_element_increases(input: Vec<i32>) -> i32 {
    let mut previous: i32 = -1;
    let mut sum;
    let mut total = 0;
    for (a, b, c) in input.iter().tuple_windows() {
        sum = a + b + c;
        if previous > 0 && sum > previous {
            total += 1;
        }
        previous = sum;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_find_increases() {
        assert_eq!(find_number_of_increases(EXAMPLE.to_vec()), 7);
    }

    #[test]
    fn test_find_3_increases() {
        assert_eq!(find_number_of_three_element_increases(EXAMPLE.to_vec()), 5);
    }
}
