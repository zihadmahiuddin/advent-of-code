use std::collections::BTreeMap;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1},
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    received_numbers: Vec<u32>,
}

impl Card {
    fn matching_numbers(&self) -> Vec<u32> {
        self.received_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .copied()
            .collect()
    }
}

#[derive(Debug)]
struct Game {
    cards: Vec<Card>,
}

impl Game {
    fn final_card_count(&self) -> usize {
        let mut card_counts = self
            .cards
            .iter()
            .map(|card| (card.id, (card, 1)))
            .collect::<BTreeMap<_, _>>();

        card_counts
            .keys()
            .copied()
            .collect_vec()
            .iter()
            .for_each(|card_id| {
                card_counts.get(&card_id).copied().map(|(card, count)| {
                    let matching_numbers = card.matching_numbers().len();
                    (0..count).for_each(|_| {
                        (1..=matching_numbers).for_each(|i| {
                            card_counts.entry(card_id + i as u32).and_modify(|(_, c)| {
                                *c += 1;
                            });
                        })
                    });
                });
            });

        card_counts.values().map(|(_, c)| c).sum()
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
    let game = Game { cards };
    Ok(game.final_card_count().to_string())
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
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
