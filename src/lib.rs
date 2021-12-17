use std::fs;
use std::io;
use std::path;

pub fn parse_input_file<P: AsRef<path::Path>>(path: P) -> Vec<i32> {
    let file = fs::File::open(path).expect("Couldn't open file");
    let reader = io::BufReader::new(file);

    parse_input_reader(reader)
}

pub fn parse_input_reader(reader: impl io::BufRead) -> Vec<i32> {
    // TODO: Look into better error handling
    let lines = reader.lines();
    let numbers = lines
        .map(|l| {
            l.expect("Couldn't read a line from input")
                .parse::<i32>()
                .expect("Couldn't convert a number to integer")
        })
        .collect();

    numbers
}
