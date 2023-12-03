use std::collections::HashMap;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32, multi::separated_list1,
    sequence::separated_pair, IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Cube {
    color: CubeColor,
    count: u32,
}

#[derive(Debug)]
struct Game {
    cube_subsets: Vec<Vec<Cube>>,
}

impl Game {
    fn min_cube_counts(&self) -> HashMap<CubeColor, u32> {
        let mut min_cube_counts = HashMap::new();

        for subset in &self.cube_subsets {
            for cube in subset {
                min_cube_counts
                    .entry(cube.color)
                    .and_modify(|a: &mut u32| {
                        *a = (*a).max(cube.count);
                    })
                    .or_insert(cube.count);
            }
        }

        min_cube_counts
    }
}

fn parse_cube_color(input: &str) -> IResult<&str, CubeColor> {
    let (input, color) = alt((tag("red"), tag("green"), tag("blue"))).parse(input)?;
    let color = match color {
        "red" => CubeColor::Red,
        "green" => CubeColor::Green,
        "blue" => CubeColor::Blue,
        _ => unreachable!("nom's tag() parser makes sure we don't parse any other values"),
    };

    Ok((input, color))
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, (count, color)) = separated_pair(u32, tag(" "), parse_cube_color).parse(input)?;
    Ok((input, Cube { color, count }))
}

fn parse_cube_subset(input: &str) -> IResult<&str, Vec<Cube>> {
    separated_list1(tag(", "), parse_cube).parse(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ").parse(input)?;
    let (input, _game_id) = u32(input)?;
    let (input, _) = tag(": ").parse(input)?;
    let (input, cube_subsets) = separated_list1(tag("; "), parse_cube_subset)(input)?;
    Ok((input, Game { cube_subsets }))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .lines()
        .map(|line| parse_game(line).expect("input to be properly formatted").1)
        .map(|game| {
            game.min_cube_counts()
                .into_values()
                .fold(1, |acc, x| acc * x)
        })
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
