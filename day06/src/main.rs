use std::fs;
use std::collections::HashSet;

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
}
