use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1},
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    received_numbers: Vec<u32>,
}

impl Card {
    fn points(&self) -> u32 {
        self.received_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
            .checked_sub(1)
            .and_then(|n| n.try_into().ok())
            .map(|n| 2u32.pow(n))
            .unwrap_or_default()
    }
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, card_id) = nom::character::complete::u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    // the first number can be a single digit, in which case there will be 1 extra space in front
    let (input, _) = space0(input)?;
    let (input, winning_numbers) =
        nom::multi::separated_list1(space1, nom::character::complete::u32)(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = space1(input)?;
    let (input, received_numbers) =
        nom::multi::separated_list1(space1, nom::character::complete::u32)(input)?;

    Ok((
        input,
        Card {
            id: card_id,
            winning_numbers,
            received_numbers,
        },
    ))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let cards = input
        .lines()
        .map(|line| parse_card(line).map(|(_, card)| card))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let points = cards.iter().map(|card| card.points()).sum::<u32>();
    Ok(points.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
