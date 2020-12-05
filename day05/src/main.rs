use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let highest = file.lines().map(|x| x.unwrap()).fold(0, |highest, line| {
        let (row, column) = decode(&line);
        let id = row * 8 + column;

        if id > highest {
            id
        } else {
            highest
        }
    });

    println!("{}", highest);
}

fn decode(pass: &str) -> (i32, i32) {
    let row = pass.chars().take(7).enumerate().fold(0, |row, (step, char)| {
        if char == 'B' {
            row + 2_i32.pow(6 - step as u32)
        } else {
            row
        }
    });

    let column = pass.chars().skip(7).take(3).enumerate().fold(0, |row, (step, char)| {
        if char == 'R' {
            row + 2_i32.pow(2 - step as u32)
        } else {
            row
        }
    });

    (row, column)
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut seats = HashSet::new();
    let highest = file.lines().map(|x| x.unwrap()).fold(0, |highest, line| {
        let (row, column) = decode(&line);
        let id = row * 8 + column;

        seats.insert(id);

        if id > highest {
            id
        } else {
            highest
        }
    });

    for i in 1..highest {
        if seats.contains(&(i - 1)) && seats.contains(&(i + 1)) && !seats.contains(&i) {
            println!("{}", i);
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!((44, 5), decode("FBFBBFFRLR"));
        assert_eq!((70, 7), decode("BFFFBBFRRR"));
        assert_eq!((14, 7), decode("FFFBBBFRRR"));
        assert_eq!((102, 4), decode("BBFFBBFRLL"));
    }
}
