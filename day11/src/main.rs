use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut seats = file.lines()
        .map(|x| x.unwrap())
        .enumerate()
        .fold(HashMap::new(), |mut accum, (y, line)| {
            for (x, char) in line.chars().enumerate() {
                accum.insert((x as i32, y as i32), char);
            }

            accum
        });

    let deltas = [(0, 1), (1, 0), (1, 1), (0, -1), (-1, 0), (-1, -1), (1, -1), (-1, 1)];
    loop {
        let mut altered_seats = seats.clone();

        for ((x, y), point) in seats.iter() {
            let neighbors = deltas.iter().fold(0, |accum, (dx, dy)| {
                accum + match seats.get(&(x + dx, y + dy)).unwrap_or(&'.') {
                    '#' => 1,
                    _ => 0,
                }
            });

            if *point == 'L' && neighbors == 0 {
                altered_seats.insert((*x, *y), '#');
            } else if *point == '#' && neighbors >= 4 {
                altered_seats.insert((*x, *y), 'L');
            }
        }

        let same = seats == altered_seats;
        seats = altered_seats;
        if same {
            break;
        }
    }

    let occupied = seats.values().filter(|x| **x == '#').count();
    println!("{}", occupied);
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut seats = file.lines()
        .map(|x| x.unwrap())
        .enumerate()
        .fold(HashMap::new(), |mut accum, (y, line)| {
            for (x, char) in line.chars().enumerate() {
                accum.insert((x as i32, y as i32), char);
            }

            accum
        });

    let deltas = [(0, 1), (1, 0), (1, 1), (0, -1), (-1, 0), (-1, -1), (1, -1), (-1, 1)];
    loop {
        let mut altered_seats = seats.clone();

        for ((x, y), point) in seats.iter() {
            if *point != '.' {
                let neighbors = deltas.iter().fold(0, |mut accum, (dx, dy)| {
                    let mut target = (x + dx, y + dy);
                    while let Some(char) = seats.get(&target) {
                        accum += match char {
                            '#' => 1,
                            _ => 0,
                        };
                        if *char == '#' || *char == 'L' {
                            break;
                        }
                        target = (target.0 + dx, target.1 + dy);
                    }

                    accum
                });

                if *point == 'L' && neighbors == 0 {
                    altered_seats.insert((*x, *y), '#');
                } else if *point == '#' && neighbors >= 5 {
                    altered_seats.insert((*x, *y), 'L');
                }
            }
        }

        let same = seats == altered_seats;
        seats = altered_seats;
        if same {
            break;
        }
    }

    let occupied = seats.values().filter(|x| **x == '#').count();
    println!("{}", occupied);
}
