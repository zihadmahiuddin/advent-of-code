use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .lines()
        .flat_map(|line| {
            let (first, second) = process_line(line);
            format!("{}{}", first, second).parse::<u32>()
        })
        .sum::<u32>()
        .to_string())
}

fn process_line(line: &str) -> (usize, usize) {
    const PATTERNS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let line = line.to_string();

    let found_nums = (0..line.len())
        .filter_map(|i| {
            let slice = &line[i..];
            slice
                .chars()
                .next()
                .and_then(|c| c.to_digit(10).map(|n| n as usize))
                .or_else(|| {
                    PATTERNS
                        .iter()
                        .enumerate()
                        .find(|(_, pattern)| slice.starts_with(*pattern))
                        .map(|(num, _)| num)
                })
        })
        .collect_vec();

    let first = found_nums.first().expect("at least 1 digit to be present");
    let second = found_nums.last().unwrap_or(first);

    (*first, *second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);

        let input = "oneight";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
