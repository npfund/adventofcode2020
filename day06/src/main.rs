use std::fs;
use std::collections::{HashSet, HashMap};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = fs::read_to_string("input.txt").unwrap();

    let total = file.split("\n\n")
        .map(|group| {
            group.replace("\n", "")
                .chars()
                .fold(HashSet::new(), |mut accum, char| {
                    accum.insert(char);
                    accum
                }).len()
        })
        .sum::<usize>();

    println!("{}", total);
}

fn part2() {
    let file = fs::read_to_string("input.txt").unwrap();

    let total = file.split("\n\n")
        .map(|group| {
            let people = group.lines().count();
            group.replace("\n", "")
                .chars()
                .fold(HashMap::new(), |mut accum, char| {
                    let entry = accum.entry(char).or_insert(0_usize);
                    *entry += 1;
                    accum
                })
                .iter()
                .filter(|(_k, v)| {
                    **v == people
                }).count()
        })
        .sum::<usize>();

    println!("{}", total);
}
