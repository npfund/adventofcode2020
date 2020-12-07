use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let outer_regex = Regex::new("(?P<outer>[a-z ]+) bags contain").unwrap();
    let inner_regex = Regex::new("(?:[0-9]+ (?P<inner>[a-z ]+) bag(?:s)?,?)+|(?:no other bags)").unwrap();
    let color_map: HashMap<String, Vec<String>> = file.lines()
        .map(|x| x.unwrap())
        .fold(HashMap::new(), |mut accum, line| {
            let outer_bag = outer_regex.captures(&line).unwrap().name("outer").unwrap().as_str().to_owned();

            for captures in inner_regex.captures_iter(&line) {
                for cap in captures.iter().skip(1) {
                    if let Some(inner_bag) = cap {
                        let entry = accum.entry(inner_bag.as_str().to_owned()).or_default();
                        (*entry).push(outer_bag.clone());
                    }
                }
            }

            accum
        });

    let mut containers = HashSet::new();
    find_container(&color_map, "shiny gold", &mut containers);

    println!("{}", containers.len());
}

fn find_container(colors: &HashMap<String, Vec<String>>, subject: &str, all_containers: &mut HashSet<String>) {
    if let Some(containers) = colors.get(subject) {
        for container in containers {
            all_containers.insert(container.clone());
            find_container(colors, container, all_containers);
        }
    }
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let color_map: HashMap<String, Vec<(i32, String)>> = get_map(file.lines()
        .map(|x| x.unwrap()));

    println!("{}", get_count(&color_map, "shiny gold") - 1);
}

fn get_map(lines: impl Iterator<Item=String>) -> HashMap<String, Vec<(i32, String)>> {
    let outer_regex = Regex::new("(?P<outer>[a-z ]+) bags contain").unwrap();
    let inner_regex = Regex::new("(?:(?P<count>[0-9]+) (?P<inner>[a-z ]+) bag(?:s)?,?)+|(?:no other bags)").unwrap();

    lines.fold(HashMap::new(), |mut accum, line| {
        let outer_bag = outer_regex.captures(&line).unwrap().name("outer").unwrap().as_str().to_owned();

        let entry = accum.entry(outer_bag).or_default();
        for captures in inner_regex.captures_iter(&line) {
            if let Some(inner) = captures.name("inner") {
                let inner_bag = inner.as_str().to_owned();
                let count = captures.name("count").unwrap().as_str().parse::<i32>().unwrap();
                (*entry).push((count, inner_bag));
            }
        }

        accum
    })
}

fn get_count(colors: &HashMap<String, Vec<(i32, String)>>, subject: &str) -> i32 {
    if let Some(inner_bags) = colors.get(subject) {
        let mut total = 1;
        for (count, bag) in inner_bags.iter() {
            total += count * get_count(colors, bag)
        }

        total
    } else {
        1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2_1() {
        let file = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let color_map: HashMap<String, Vec<(i32, String)>> = get_map(file.lines()
            .map(|x| x.to_owned())
        );

        assert_eq!(32, get_count(&color_map, "shiny gold") - 1);
    }

    #[test]
    fn part2_2() {
        let file = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        let color_map: HashMap<String, Vec<(i32, String)>> = get_map(file.lines()
            .map(|x| x.to_owned())
        );

        assert_eq!(126, get_count(&color_map, "shiny gold") - 1);
    }
}
