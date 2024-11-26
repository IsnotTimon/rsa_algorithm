use std::io;

pub fn input_two() -> Vec<u64> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let nums: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("d"))
        .collect();
    nums
}

pub fn input_single() -> u64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let input_string: u64 = input.trim().parse().expect("Wrong input");
    input_string
}
