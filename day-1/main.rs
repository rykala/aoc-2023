use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let result = read_input().iter().fold(0, |acc, x| acc + extract_number(x));
    println!("{}", result);
}

fn extract_number(input: &str) -> u32 {
    let reversed_input = input.chars().rev().collect::<String>();
    return [
        find_first_number(input),
        find_first_number(&reversed_input),
    ].join("").parse().unwrap();
}

fn find_first_number(x: &str) -> &str {
    let first_number_regex = Regex::new(r"\d").unwrap();
    let res = first_number_regex.find(x).unwrap();
    return x.get(res.start()..res.end()).unwrap();
}

fn read_input() -> Vec<String> {
    return read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
