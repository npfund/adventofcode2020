use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
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

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut adapters = file.lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>();
    adapters.sort();

    let mut map: HashMap<i32, u64> = HashMap::new();
    map.insert(0, 1);

    for a in &adapters {
        map.insert(
            *a,
            map.get(&(a - 1)).unwrap_or(&0)
                + map.get(&(a - 2)).unwrap_or(&0)
                + map.get(&(a - 3)).unwrap_or(&0),
        );
    }

    println!("{}", map.get(adapters.last().unwrap()).unwrap());
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
