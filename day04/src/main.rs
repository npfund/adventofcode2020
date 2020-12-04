use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = fs::read_to_string("input.txt").unwrap();

    let valid = file.split("\n\n").fold(0, |accum, passport| {
        if passport.contains("byr")
            && passport.contains("iyr")
            && passport.contains("eyr")
            && passport.contains("hgt")
            && passport.contains("hcl")
            && passport.contains("ecl")
            && passport.contains("pid")
        {
            accum + 1
        } else {
            accum
        }
    });

    println!("{}", valid);
}

fn part2() {
}
