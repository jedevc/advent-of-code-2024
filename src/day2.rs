use nom::{
    character::complete::{newline, space1, u64},
    multi::separated_list1,
    IResult,
};

pub fn solve(reports: Vec<Vec<u64>>) -> (u64, u64) {
    let part1 = reports.iter().filter(|report| valid_report(report)).count();

    let part2 = reports
        .iter()
        .filter(|report| {
            report.iter().enumerate().any(|(i, _)| {
                let report: Vec<u64> = report
                    .iter()
                    .take(i)
                    .chain(report.iter().skip(i + 1))
                    .map(u64::to_owned)
                    .collect();
                valid_report(&report)
            })
        })
        .count();

    (part1 as u64, part2 as u64)
}

fn valid_report(report: &[u64]) -> bool {
    (report.iter().is_sorted() || report.iter().rev().is_sorted())
        && report
            .windows(2)
            .all(|w| (1..4).contains(&(w[0].abs_diff(w[1]))))
}

pub fn parse(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(newline, parse_line)(input)
}

pub fn parse_line(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, u64)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_day2() {
        let input = util::read_resource("day2.example.txt").unwrap();
        let (_, data) = parse(&input).unwrap();
        let (part1, part2) = solve(data);
        assert_eq!(part1, 2);
        assert_eq!(part2, 4);
    }
}
