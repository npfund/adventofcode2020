use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Turn {
    Right,
    Left,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Command {
    Forward,
    Move(Direction),
    Rotate(Turn),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Instruction {
    command: Command,
    value: i32,
}

impl From<String> for Instruction {
    fn from(input: String) -> Self {
        use Command::*;
        use Direction::*;
        use Turn::*;

        let (c, v) = input.split_at(1);
        let command = match c {
            "N" => Move(North),
            "E" => Move(East),
            "S" => Move(South),
            "W" => Move(West),
            "R" => Rotate(Right),
            "L" => Rotate(Left),
            "F" => Forward,
            _ => panic!(),
        };

        let value = v.parse::<i32>().unwrap();

        Instruction {
            command,
            value,
        }
    }
}


fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut facing = Direction::East;
    let mut x = 0;
    let mut y = 0;
    for instruction in file.lines().map(|x| Instruction::from(x.unwrap())) {
        match instruction.command {
            Command::Forward => {
                let delta = delta(facing);
                x += delta.0 * instruction.value;
                y += delta.1 * instruction.value;
            }
            Command::Move(direction) => {
                let delta = delta(direction);
                x += delta.0 * instruction.value;
                y += delta.1 * instruction.value;
            }
            Command::Rotate(turn) => {
                let value = match turn {
                    Turn::Right => instruction.value,
                    Turn::Left => 360 - instruction.value,
                };
                let steps = value / 90;

                facing = match (match facing {
                    Direction::North => 0,
                    Direction::East => 1,
                    Direction::South => 2,
                    Direction::West => 3,
                } + steps) % 4 {
                    0 => Direction::North,
                    1 => Direction::East,
                    2 => Direction::South,
                    3 => Direction::West,
                    _ => panic!(),
                };
            }
        }
    }

    println!("|{}| + |{}| = {}", x, y, x.abs() + y.abs());
}

fn delta(dir: Direction) -> (i32, i32) {
    match dir {
        Direction::North => (0, 1),
        Direction::East => (1, 0),
        Direction::South => (0, -1),
        Direction::West => (-1, 0),
    }
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut x = 0;
    let mut y = 0;
    let mut waypoint_dx = 10;
    let mut waypoint_dy = 1;
    for instruction in file.lines().map(|x| Instruction::from(x.unwrap())) {
        match instruction.command {
            Command::Forward => {
                x += waypoint_dx * instruction.value;
                y += waypoint_dy * instruction.value;
            }
            Command::Move(direction) => {
                let delta = delta(direction);
                waypoint_dx += delta.0 * instruction.value;
                waypoint_dy += delta.1 * instruction.value;
            }
            Command::Rotate(turn) => {
                let value = match turn {
                    Turn::Right => instruction.value,
                    Turn::Left => 360 - instruction.value,
                };
                let steps = (value / 90) % 4;

                let (new_waypoint_dx, new_waypoint_dy) = match steps {
                    0 => (waypoint_dx, waypoint_dy),
                    1 => (waypoint_dy, -waypoint_dx),
                    2 => (-waypoint_dx, -waypoint_dy),
                    3 => (-waypoint_dy, waypoint_dx),
                    _ => panic!()
                };

                waypoint_dx = new_waypoint_dx;
                waypoint_dy = new_waypoint_dy;
            }
        }
    }

    println!("|{}| + |{}| = {}", x, y, x.abs() + y.abs());
}
