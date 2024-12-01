use std::fs;
use std::io;
use std::path::Path;

pub fn read_resource(filename: &str) -> io::Result<String> {
    let path = Path::new("resources").join(filename);
    let mut data = fs::read_to_string(path)?;
    let trimmed = data.trim_end();
    data.truncate(trimmed.len());
    Ok(data)
}

// pub fn parse_and_solve<T, R1, R2, F1, F2, P, S>(
//     input: &str,
//     parser: P,
//     solver: S,
// ) -> Result<(F1, F2), Box<dyn Error + '_>>
// where
//     P: Fn(&str) -> IResult<&str, T>,
//     S: Fn(T) -> (R1, R2),
//     F1: From<R1>,
//     F2: From<R2>,
// {
//     let (_, data) = parser(input)?;
//     let (part1, part2) = solver(data);
//     Ok((part1.into(), part2.into()))
// }
