use std::ops::Range;

use itertools::Itertools;

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
struct EngineGear {
    pos: Position,
    part_nums: (usize, usize),
}

#[derive(Debug)]
struct Engine {
    grid: Vec<Vec<char>>,
    numbers: Vec<EngineNumber>,
    gears: Vec<EngineGear>,
}

impl Engine {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let numbers = Self::get_numbers(&grid);
        let gears = Self::get_gears(&grid, &numbers);
        Self {
            grid,
            numbers,
            gears,
        }
    }

    fn get_number(grid: &Vec<Vec<char>>, pos: Position) -> Option<EngineNumber> {
        let Position { x, y } = pos;
        let Some(row) = grid.get(y) else {
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

    fn get_numbers(grid: &Vec<Vec<char>>) -> Vec<EngineNumber> {
        grid.iter()
            .enumerate()
            .flat_map(|(y, line)| {
                (0..line.len())
                    .scan(0, move |x, _idx| {
                        if let Some(engine_number) = Self::get_number(&grid, Position { x: *x, y })
                        {
                            *x += engine_number.range.end - engine_number.range.start;
                            Some(Some(engine_number))
                        } else {
                            *x += 1;
                            Some(None)
                        }
                    })
                    .flatten()
            })
            .filter(|num| Self::is_part_number(grid, num))
            .collect()
    }

    fn is_part_number(grid: &Vec<Vec<char>>, num: &EngineNumber) -> bool {
        let y = num.row;

        // check if the character to the direct left is a symbol
        let left = num
            .range
            .start
            .checked_sub(1)
            .and_then(|x| grid.get(y).and_then(|row| row.get(x)))
            .is_some_and(is_symbol);

        // check if the character to the direct right is a symbol
        let right = grid
            .get(y)
            .and_then(|row| row.get(num.range.end))
            .is_some_and(is_symbol);

        // check if the character range above the current line has a symbol
        // this includes the range of the whole number and also the top-left and top-right diagonals
        let top = y.checked_sub(1).is_some_and(|y| {
            grid.get(y).is_some_and(|row| {
                let search_range = num.range.start.saturating_sub(1)
                    ..num.range.end.saturating_add(1).min(row.len());
                let search_str = &row[search_range.clone()];
                search_str.iter().any(is_symbol)
            })
        });

        // check if the character range below the current line has a symbol
        // this includes the range of the whole number and also the bottom-left and bottom-right diagonals
        let bottom = y.checked_add(1).is_some_and(|y| {
            grid.get(y).is_some_and(|row| {
                let search_range = num.range.start.saturating_sub(1)
                    ..num.range.end.saturating_add(1).min(row.len());
                let search_str = &row[search_range.clone()];
                search_str.iter().any(is_symbol)
            })
        });

        left || right || top || bottom
    }

    fn get_gears(grid: &Vec<Vec<char>>, nums: &Vec<EngineNumber>) -> Vec<EngineGear> {
        grid.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &c)| c == '*')
                    .map(move |(x, _)| Position { y, x })
            })
            .filter_map(|pos| {
                let mut nums = nums.iter().filter(|num| {
                    if !(num.row.saturating_sub(1)..=num.row.saturating_add(1)).contains(&pos.y) {
                        return false;
                    }
                    let left = num.row == pos.y && num.range.start.saturating_sub(1) == pos.x;
                    let right = num.row == pos.y && num.range.end == pos.x;

                    let top = num.row.checked_sub(1).is_some_and(|y| {
                        grid.get(y).is_some_and(|row| {
                            (num.range.start.saturating_sub(1)
                                ..num.range.end.saturating_add(1).min(row.len()))
                                .contains(&pos.x)
                        })
                    });

                    let bottom = num.row.checked_add(1).is_some_and(|y| {
                        grid.get(y).is_some_and(|row| {
                            (num.range.start.saturating_sub(1)
                                ..num.range.end.saturating_add(1).min(row.len()))
                                .contains(&pos.x)
                        })
                    });

                    left || right || top || bottom
                });
                let (first, second) = nums.next_tuple()?;
                if nums.next().is_none() {
                    // if there are no more items that means this engine has exactly 2 adjacent part numbers
                    Some(EngineGear {
                        pos,
                        part_nums: (first.number, second.number),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn gears(&self) -> &[EngineGear] {
        &self.gears
    }
}

fn is_symbol(c: &char) -> bool {
    !c.is_digit(10) && *c != '.'
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid = input.lines().map(|line| line.chars().collect()).collect();
    let engine = Engine::new(grid);
    let gears = engine.gears();
    let sum = gears
        .iter()
        .map(|gear| gear.part_nums.0 * gear.part_nums.1)
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
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
