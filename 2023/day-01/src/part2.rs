use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .lines()
        .flat_map(|line| {
            let (first, second) = process_line(line);
            eprintln!("{} => ({}, {})", line, first, second);
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

    let mut found_nums = vec![];

    // start to end
    let mut index = 0;
    'root: while index < line.len() {
        if let Ok(digit) = line[index..index + 1].parse() {
            if digit <= 9 && digit >= 1 {
                found_nums.push(digit);
                break;
            }
        }

        for i in 3..=5 {
            let range = index..index + i;
            if range.end >= line.len() {
                continue;
            }

            let slice = &line[range.clone()].to_owned();
            for (num, pattern) in PATTERNS.iter().enumerate() {
                if &slice == pattern {
                    found_nums.push(num);
                    break 'root;
                }
            }
        }

        index += 1;
    }

    // end to start
    let mut index = line.len();
    'root: while index > 0 {
        if let Ok(digit) = line[index - 1..index].parse() {
            if digit <= 9 && digit >= 1 {
                found_nums.push(digit);
                break 'root;
            }
        }

        for i in 3..=5 {
            let range = index.saturating_sub(i)..index;
            if range.start < 1 {
                continue;
            }

            let slice = &line[range.clone()].to_owned();
            for (num, pattern) in PATTERNS.iter().enumerate() {
                if &slice == pattern {
                    found_nums.push(num);
                    break 'root;
                }
            }
        }

        index -= 1;
    }

    let first = found_nums.first().expect("at least 1 digit to be present");
    let second = found_nums.get(1).unwrap_or(first);

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
