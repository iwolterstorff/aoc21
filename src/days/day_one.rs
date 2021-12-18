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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_increases() {
        let example = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(find_number_of_increases(example), 7);
    }
}
