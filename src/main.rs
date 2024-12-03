mod day1;
mod day2;
mod day3;
mod util;

fn main() {
    let input = util::read_resource("day1.txt").unwrap();
    let (_, data) = day1::parse(&input).unwrap();
    let (part1, part2) = day1::solve(data);
    println!("day 1, part 1: {}", part1);
    println!("day 1, part 2: {}", part2);

    let input = util::read_resource("day2.txt").unwrap();
    let (_, data) = day2::parse(&input).unwrap();
    let (part1, part2) = day2::solve(data);
    println!("day 2, part 1: {}", part1);
    println!("day 2, part 2: {}", part2);

    let input = util::read_resource("day3.txt").unwrap();
    let (_, data) = day3::parse(&input).unwrap();
    let (part1, part2) = day3::solve(data);
    println!("day 3, part 1: {}", part1);
    println!("day 3, part 2: {}", part2);
}
