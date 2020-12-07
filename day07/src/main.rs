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
}
