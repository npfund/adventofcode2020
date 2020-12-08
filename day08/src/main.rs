use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet};

enum Operation {
    Acc,
    Jmp,
    Nop,
}

impl From<&str> for Operation {
    fn from(input: &str) -> Self {
        match input {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            "nop" => Operation::Nop,
            _ => panic!(),
        }
    }
}

struct Instruction {
    operation: Operation,
    parameter: i32,
}

impl From<String> for Instruction {
    fn from(input: String) -> Self {
        let (operation, param) = input.split_at(input.find(" ").unwrap());

        Instruction {
            operation: operation.into(),
            parameter: param.trim_start().parse().unwrap()
        }
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let instructions: Vec<Instruction> = file.lines().map(|x| x.unwrap().into()).collect();

    let mut visited = HashSet::new();

    let mut pointer: i32 = 0;
    let mut accum = 0;
    loop {
        let inserted = visited.insert(pointer);
        if !inserted {
            break;
        }

        let instruction = &instructions[pointer as usize];
        match instruction.operation {
            Operation::Acc => accum += instruction.parameter,
            Operation::Jmp => { pointer += instruction.parameter; continue; }
            Operation::Nop => {}
        }

        pointer += 1;
    }

    println!("{}", accum);
}

fn part2() {

}
