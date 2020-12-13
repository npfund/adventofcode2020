fn main() {
    part1();
    part2();
}

fn part1() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let departure = file.split("\n").next().unwrap().parse::<i32>().unwrap();
    let ids: Vec<i32> = file.split("\n")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(str::parse)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let best = ids.iter().fold((0, f32::MAX), |(accum, distance), id| {
        let div = departure as f32 / *id as f32;
        let nearest = div.ceil();
        if nearest - div < distance {
            (*id, nearest - div)
        } else {
            (accum, distance)
        }
    }).0;

    let time = (departure as f32 / best as f32).ceil() as i32 * best;
    println!("{} - {} = {}", time, departure, time - departure);
    println!("{} * {} = {}", time - departure, best, (time - departure) * best);
}

fn part2() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let ids = file.split("\n")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|(_, s)| s != &"x")
        .map(|(position, id)| (position as i64, id.parse().unwrap()))
        .collect::<Vec<_>>();

    let modulopods = ids.iter().map(|&(_, id)| id).collect::<Vec<_>>();
    let diffs = ids.iter().map(|&(position, id)| id - position).collect::<Vec<_>>();

    let product = modulopods.iter().product::<i64>();
    let mut sum = 0;
    for (&diff, modulus) in diffs.iter().zip(modulopods) {
        let p = product / modulus;
        sum += diff * mod_inv(p, modulus).unwrap() * p
    }

    println!("{}", sum % product);
}

// from: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
