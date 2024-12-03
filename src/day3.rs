use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, u64},
    multi::{many1, many_till},
    sequence::tuple,
    IResult,
};

pub fn solve(data: Vec<Instruction>) -> (u64, u64) {
    let part1 = data
        .iter()
        .map(|inst| match inst {
            Instruction::Mul(n, m) => n * m,
            _ => 0,
        })
        .sum();

    let (part2, _) = data
        .iter()
        .fold((0, true), |(total, enabled), inst| match inst {
            Instruction::Mul(n, m) => (total + if enabled { n * m } else { 0 }, enabled),
            Instruction::Do => (total, true),
            Instruction::Dont => (total, false),
        });

    (part1, part2)
}

pub fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(|input| {
        let (input, (_, nm)) = many_till(anychar, parse_instruction)(input)?;
        Ok((input, nm))
    })(input)
}

pub fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_do, parse_mul))(input)
}

pub fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, n, _, m, _)) = tuple((tag("mul("), u64, char(','), u64, tag(")")))(input)?;
    Ok((input, Instruction::Mul(n, m)))
}

pub fn parse_do(input: &str) -> IResult<&str, Instruction> {
    let (input, (res, _)) = tuple((alt((tag("don't"), tag("do"))), tag("()")))(input)?;
    let instruction = match res {
        "do" => Instruction::Do,
        "don't" => Instruction::Dont,
        _ => unreachable!(),
    };
    Ok((input, instruction))
}

#[derive(Debug)]
pub enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_day3() {
        let input = util::read_resource("day3-part1.example.txt").unwrap();
        let (_, data) = parse(&input).unwrap();
        let (part1, _) = solve(data);
        assert_eq!(part1, 161);

        let input = util::read_resource("day3-part2.example.txt").unwrap();
        let (_, data) = parse(&input).unwrap();
        let (_, part2) = solve(data);
        assert_eq!(part2, 48);
    }
}
