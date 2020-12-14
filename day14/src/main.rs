use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mask_regex = Regex::new("mask = (?P<mask>[01X]+)").unwrap();
    let mem_regex = Regex::new(r"mem\[(?P<register>[0-9]+)] = (?P<value>[0-9]+)").unwrap();
    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut mask = String::new();
    for line in file.lines().map(Result::unwrap) {
        if let Some(captures) = mem_regex.captures(&line) {
            let register = captures.name("register").unwrap().as_str().parse().unwrap();
            let value = captures.name("value").unwrap().as_str().parse().unwrap();

            let value = mask.chars().rev().enumerate().fold(value, |accum, (i, char)| {
                let bit = 2u64.pow(i as u32);
                if char == '0' {
                    accum & !bit
                } else if char == '1' {
                    accum | bit
                } else {
                    accum
                }
            });

            memory.insert(register, value);
        } else if let Some(captures) = mask_regex.captures(&line) {
            mask = captures.name("mask").unwrap().as_str().to_owned();
        }
    }

    let sum = memory.values().sum::<u64>();
    println!("{}", sum);
}

fn part2() {
}
