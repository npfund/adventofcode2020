use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let forest = file.lines().map(|x| x.unwrap()).collect::<Vec<_>>();

    let mut x = 0;
    let mut trees = 0;
    for line in &forest {
        if line.chars().nth(x % line.len()).unwrap() == '#' {
            trees += 1;
        }

        x += 3;
    }

    println!("{}", trees);
}

fn part2() {
}
