use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let numbers = file.lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>();

    for x in &numbers {
        for y in &numbers {
            if x + y == 2020 {
                println!("{} * {} = {}", x, y, x * y);
                return;
            }
        }
    }
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let numbers = file.lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>();

    for x in &numbers {
        for y in &numbers {
            for z in &numbers {
                if x + y + z == 2020 {
                    println!("{} * {} * {} = {}", x, y, z, x * y * z);
                    return;
                }
            }
        }
    }
}
