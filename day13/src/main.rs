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
}
