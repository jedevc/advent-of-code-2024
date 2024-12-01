mod day1;
mod util;

fn main() {
    let input = util::read_resource("day1.txt").unwrap();
    let (_, data) = day1::parse(&input).unwrap();
    let (part1, part2) = day1::solve(data);
    println!("day 1, part 1: {}", part1);
    println!("day 1, part 2: {}", part2);
}
