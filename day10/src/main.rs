use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut adapters = file.lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>();
    adapters.sort();

    let mut ones = 0;
    let mut threes = 1;
    let mut previous = 0;
    for a in adapters {
        if a - previous == 1 {
            ones += 1;
        } else {
            threes += 1;
        }

        previous = a;
    }

    println!("{} * {} = {}", ones, threes, ones * threes);
}

#[cfg(test)]
mod test {
    #[test]
    fn part1_1() {
        let mut adapters = vec![
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4
        ];
        adapters.sort();

        let mut ones = 0;
        let mut threes = 1;
        let mut previous = 0;
        for a in adapters {
            if a - previous == 1 {
                ones += 1;
            } else {
                threes += 1;
            }

            previous = a;
        }

        assert_eq!(7, ones);
        assert_eq!(5, threes);
    }
}
