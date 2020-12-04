use std::fs;
use regex::Regex;

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
    let file = fs::read_to_string("input.txt").unwrap();

    let byr_regex = Regex::new("byr:([0-9]{4})( |$)").unwrap();
    let iyr_regex = Regex::new("iyr:([0-9]{4})( |$)").unwrap();
    let eyr_regex = Regex::new("eyr:([0-9]{4})( |$)").unwrap();
    let hgt_regex = Regex::new("hgt:([0-9]+)(cm|in)( |$)").unwrap();
    let hcl_regex = Regex::new("hcl:#[0-9a-f]{6}( |$)").unwrap();
    let ecl_regex = Regex::new("ecl:(amb|blu|brn|gry|grn|hzl|oth)( |$)").unwrap();
    let pid_regex = Regex::new("pid:[0-9]{9}( |$)").unwrap();

    let valid = file.split("\n\n")
        .map(|p| p.replace("\n", " "))
        .fold(0, |accum, passport| {
            let byr_valid = if let Some(captures) = byr_regex.captures(&passport) {
                let value = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
                1920 <= value && value <= 2002
            } else {
                false
            };

            let iyr_valid = if let Some(captures) = iyr_regex.captures(&passport) {
                let value = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
                2010 <= value && value <= 2020
            } else {
                false
            };

            let eyr_valid = if let Some(captures) = eyr_regex.captures(&passport) {
                let value = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
                2020 <= value && value <= 2030
            } else {
                false
            };

            let hgt_valid = if let Some(captures) = hgt_regex.captures(&passport) {
                let value = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let unit = captures.get(2).unwrap().as_str();
                if unit == "cm" {
                    150 <= value && value <= 193
                } else {
                    59 <= value && value <= 76
                }
            } else {
                false
            };

            let non_capturing = hcl_regex.is_match(&passport)
                && ecl_regex.is_match(&passport)
                && pid_regex.is_match(&passport);

            if byr_valid && iyr_valid && eyr_valid && hgt_valid && non_capturing {
                println!("{}\n", passport);
                accum + 1
            } else {
                accum
            }
        });

    println!("{}", valid);
}
