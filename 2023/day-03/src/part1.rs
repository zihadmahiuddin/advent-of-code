use std::ops::Range;

use crate::custom_error::AocError;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct EngineNumber {
    row: usize,
    range: Range<usize>,
    number: usize,
}

#[derive(Debug)]
struct Engine {
    grid: Vec<Vec<char>>,
}

impl Engine {
    fn get_number(&self, pos: Position) -> Option<EngineNumber> {
        let Position { x, y } = pos;
        let Some(row) = self.grid.get(y) else {
            return None;
        };
        let Some(val) = row.get(x) else {
            return None;
        };

        if !val.is_digit(10) {
            return None;
        }

        let mut num_str = String::new();

        // check the left side of the current X position
        let mut left_index = 0;
        for i in (0..x).rev() {
            let Some(v) = row.get(i).filter(|x| x.is_digit(10)) else {
                break;
            };

            left_index += 1;

            num_str.push(*v);
        }

        // push the character on the current position to the right of the ones found on the left side
        num_str.push(*val);

        // check the right side of the current X position
        let mut right_index = 0;
        for i in (x + 1)..row.len() {
            let Some(v) = row.get(i).filter(|x| x.is_digit(10)) else {
                break;
            };

            right_index += 1;

            num_str.push(*v);
        }

        num_str.parse().ok().map(|num| EngineNumber {
            row: y,
            range: (x - left_index)..(x + right_index + 1),
            number: num,
        })
    }

    fn get_numbers(&self) -> Vec<EngineNumber> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                (0..line.len())
                    .scan(0, move |x, _idx| {
                        if let Some(engine_number) = self.get_number(Position { x: *x, y }) {
                            *x += engine_number.range.end - engine_number.range.start;
                            Some(Some(engine_number))
                        } else {
                            *x += 1;
                            Some(None)
                        }
                    })
                    .flatten()
            })
            .collect::<Vec<_>>()
    }

    fn is_part_number(&self, num: &EngineNumber) -> bool {
        let y = num.row;

        // check if the character to the direct left is a symbol
        let left = num
            .range
            .start
            .checked_sub(1)
            .and_then(|x| self.grid.get(y).and_then(|row| row.get(x)))
            .is_some_and(is_symbol);

        // check if the character to the direct right is a symbol
        let right = self
            .grid
            .get(y)
            .and_then(|row| row.get(num.range.end))
            .is_some_and(is_symbol);

        // check if the character range above the current line has a symbol
        // this includes the range of the whole number and also the top-left and top-right diagonals
        let top = y.checked_sub(1).is_some_and(|y| {
            self.grid.get(y).is_some_and(|row| {
                let search_range = num.range.start.saturating_sub(1)
                    ..num.range.end.saturating_add(1).min(row.len());
                let search_str = &row[search_range.clone()];
                search_str.iter().any(is_symbol)
            })
        });

        // check if the character range below the current line has a symbol
        // this includes the range of the whole number and also the bottom-left and bottom-right diagonals
        let bottom = y.checked_add(1).is_some_and(|y| {
            self.grid.get(y).is_some_and(|row| {
                let search_range = num.range.start.saturating_sub(1)
                    ..num.range.end.saturating_add(1).min(row.len());
                let search_str = &row[search_range.clone()];
                search_str.iter().any(is_symbol)
            })
        });

        left || right || top || bottom
    }
}

fn is_symbol(c: &char) -> bool {
    !c.is_digit(10) && *c != '.'
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid = input.lines().map(|line| line.chars().collect()).collect();
    let engine = Engine { grid };
    let sum = engine
        .get_numbers()
        .into_iter()
        .filter(|num| engine.is_part_number(num))
        .map(|num| num.number)
        .sum::<usize>();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
