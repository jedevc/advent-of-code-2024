use nom::{
    character::complete::{newline, space1, u64},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;
use std::iter::zip;

pub fn solve((v1s, v2s): (Vec<u64>, Vec<u64>)) -> (u64, u64) {
    let (mut v1s, mut v2s) = (v1s.to_vec(), v2s.to_vec());
    v1s.sort();
    v2s.sort();
    let part1 = zip(&v1s, &v2s).map(|(v1, v2)| v1.abs_diff(*v2)).sum();

    let lookup = v2s.iter().fold(HashMap::new(), |mut h, v| {
        h.insert(v, h.get(v).unwrap_or(&0) + 1);
        h
    });
    let part2 = v1s.iter().map(|v| v * lookup.get(v).unwrap_or(&0)).sum();

    (part1, part2)
}

pub fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    let (input, vs) = separated_list1(newline, parse_line)(input)?;
    let vs = vs
        .iter()
        .fold((Vec::new(), Vec::new()), |(mut v1s, mut v2s), (v1, v2)| {
            v1s.push(*v1);
            v2s.push(*v2);
            (v1s, v2s)
        });
    Ok((input, vs))
}

fn parse_line(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, (v1, _, v2)) = tuple((u64, space1, u64))(input)?;
    Ok((input, (v1, v2)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_day1() {
        let input = util::read_resource("day1.example.txt").unwrap();
        let (_, data) = parse(&input).unwrap();
        let (part1, part2) = solve(data);
        assert_eq!(part1, 11);
        assert_eq!(part2, 31);
    }
}
