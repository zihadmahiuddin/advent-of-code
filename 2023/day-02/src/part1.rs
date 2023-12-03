use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32, multi::separated_list1,
    sequence::separated_pair, IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

impl CubeColor {
    fn possible_count(&self) -> u32 {
        match self {
            Self::Red => 12,
            Self::Green => 13,
            Self::Blue => 14,
        }
    }
}

#[derive(Debug)]
struct Cube {
    color: CubeColor,
    count: u32,
}

impl Cube {
    fn is_possible(&self) -> bool {
        let possible_count = self.color.possible_count();
        self.count <= possible_count
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    cube_subsets: Vec<Vec<Cube>>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.cube_subsets
            .iter()
            .all(|subset| subset.iter().all(Cube::is_possible))
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
    let (input, game_id) = u32(input)?;
    let (input, _) = tag(": ").parse(input)?;
    let (input, cube_subsets) = separated_list1(tag("; "), parse_cube_subset)(input)?;
    Ok((
        input,
        Game {
            id: game_id,
            cube_subsets,
        },
    ))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .lines()
        .map(|line| parse_game(line).expect("input to be properly formatted").1)
        .filter(|game| game.is_possible())
        .map(|game| game.id)
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
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
