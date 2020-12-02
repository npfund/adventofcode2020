use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut valid = 0;
    let pattern = Regex::new("(?P<min>[0-9]+)-(?P<max>[0-9]+) (?P<char>[a-z]): (?P<password>[a-z]+)").unwrap();
    for line in file.lines().map(|x| x.unwrap()) {
        let captures = pattern.captures(&line).unwrap();
        let min = captures.name("min").unwrap().as_str().parse::<usize>().unwrap();
        let max = captures.name("max").unwrap().as_str().parse::<usize>().unwrap();
        let char = captures.name("char").unwrap().as_str().chars().nth(0).unwrap();
        let password = captures.name("password").unwrap().as_str();
        let matches = password.chars().filter(|&c| c == char).count();
        if min <= matches && matches <= max {
            valid += 1;
        }
    }

    println!("{} valid passwords", valid);
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut valid = 0;
    let pattern = Regex::new("(?P<first>[0-9]+)-(?P<second>[0-9]+) (?P<char>[a-z]): (?P<password>[a-z]+)").unwrap();
    for line in file.lines().map(|x| x.unwrap()) {
        let captures = pattern.captures(&line).unwrap();
        let first = captures.name("first").unwrap().as_str().parse::<usize>().unwrap() - 1;
        let second = captures.name("second").unwrap().as_str().parse::<usize>().unwrap() - 1;
        let char = captures.name("char").unwrap().as_str().chars().nth(0).unwrap();
        let password = captures.name("password").unwrap().as_str();
        if let Some(first_char) = password.chars().nth(first) {
            if let Some(second_char) = password.chars().nth(second) {
                if (first_char == char) ^ (second_char == char) {
                    valid += 1;
                }
            }
        }
    }

    println!("{} valid passwords", valid);
}
