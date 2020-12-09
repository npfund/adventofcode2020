use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

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
    let file = BufReader::new(File::open("input.txt").unwrap());

    let numbers = file.lines().map(|x| x.unwrap().parse::<i64>().unwrap()).collect::<Vec<_>>();

    let mut target = 0;
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
            target = num;
            break;
        }
    }

    let mut range = VecDeque::new();
    let mut sum = 0;
    let mut end = 0;
    while sum != target {
        if sum < target {
            range.push_back(numbers[end]);
            end += 1;
        } else if sum > target {
            range.pop_front();
        }

        sum = range.iter().sum();
    }

    let smallest = range.iter().min().unwrap();
    let largest = range.iter().max().unwrap();

    println!("{} + {} = {}", smallest, largest, smallest + largest);
}
