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
    let file = BufReader::new(File::open("input.txt").unwrap());

    let forest = file.lines().map(|x| x.unwrap()).collect::<Vec<_>>();

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut total: u64 = 1;
    for (dx, dy) in slopes.iter() {
        let mut x = 0;
        let mut y = 0;
        let mut trees = 0;
        while y < forest.len() {
            let row = &forest[y];
            if row.chars().nth(x % row.len()).unwrap() == '#' {
                trees += 1;
            }

            x += dx;
            y += dy;
        }

        println!("{}/{} {}", dx, dy, trees);
        total *= trees;
    }

    println!("{}", total);
}
