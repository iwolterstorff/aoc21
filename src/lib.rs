use std::fs;
use std::io;
use std::num::ParseIntError;
use std::path;

use anyhow::Result;

pub fn tokenize_input(reader: impl io::BufRead) -> Result<Vec<String>> {
    reader
        .lines()
        .collect::<Result<Vec<_>, io::Error>>()
        .map_err(anyhow::Error::new)
}

pub fn tokenize_input_to_integers(reader: impl io::BufRead) -> Result<Vec<i32>> {
    let lines = tokenize_input(reader)?;
    lines
        .into_iter()
        .map(|l| l.parse::<i32>())
        .collect::<Result<Vec<_>, ParseIntError>>()
        .map_err(anyhow::Error::new)
}

pub fn tokenize_file<P: AsRef<path::Path>>(path: P) -> Result<Vec<String>> {
    let file = fs::File::open(path)?;
    tokenize_input(io::BufReader::new(file))
}

pub fn tokenize_file_to_integers<P: AsRef<path::Path>>(path: P) -> Result<Vec<i32>> {
    let file = fs::File::open(path)?;
    tokenize_input_to_integers(io::BufReader::new(file))
}

pub fn nth_bit_set(number: i32, n: u32) -> bool {
    let two: i32 = 2;
    two.pow(n) & number != 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_parse_input_reader() -> Result<()> {
        let reader = io::BufReader::new("32\n77\n".as_bytes());
        assert_eq!(tokenize_input_to_integers(reader)?, vec![32, 77]);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_cannot_parse_a_number() {
        let reader = io::BufReader::new("55\ncake\n".as_bytes());
        tokenize_input_to_integers(reader).unwrap();
    }
}
