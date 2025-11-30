use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../../../puzzle_input/day_01_2020.txt")
        .expect("Failed to open puzzle input file");
    
    let reader = BufReader::new(file);
    let mut values: Vec<i32> = Vec::new();
    
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Ok(num) = line.trim().parse::<i32>() {
                values.push(num);
            }
        }
    }
    
    println!("Read {} values from puzzle input", values.len());
    println!("First 5 values: {:?}", &values[..5.min(values.len())]);
}
