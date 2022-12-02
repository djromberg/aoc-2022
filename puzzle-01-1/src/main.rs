use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut maximum = 0;
        let mut accumulator = 0;
        for line in lines {
            if let Ok(line_str) = line {
                if line_str.is_empty() {
                    if accumulator > maximum {
                        maximum = accumulator;
                    }
                    accumulator = 0;
                }
                else {
                    accumulator = accumulator + line_str.parse::<i32>().unwrap();
                }
            }
        }
        print!("Maximum calories carried: {}", maximum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
