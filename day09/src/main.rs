use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let numbers = file.lines().map(|x| x.unwrap().parse::<i64>().unwrap()).collect::<Vec<_>>();

    for i in 25..numbers.len() {
        let num = numbers[i];

        let mut found = false;
        for x in (i-25)..i {
            for y in (i-25)..i {
                if numbers[x] + numbers[y] == num {
                    found = true;
                    break;
                }
            }
        }

        if !found {
            println!("{}", num);
            break;
        }
    }
}

fn part2() {
}
