use std::{collections::BTreeMap, fmt::Display, ops::Range, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, newline, space1, u64},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, almanac) = parse_almanac(input).unwrap();

    let lowest_location = almanac
        .seeds
        .iter()
        .filter_map(|&seed| almanac.resolve_for(seed))
        .min()
        .expect("At least one item to exist.");

    Ok(lowest_location.to_string())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(Self::Seed),
            "soil" => Ok(Self::Soil),
            "fertilizer" => Ok(Self::Fertilizer),
            "water" => Ok(Self::Water),
            "light" => Ok(Self::Light),
            "temperature" => Ok(Self::Temperature),
            "humidity" => Ok(Self::Humidity),
            "location" => Ok(Self::Location),
            _ => Err(()),
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            Self::Seed => "seed",
            Self::Soil => "soil",
            Self::Fertilizer => "fertilizer",
            Self::Water => "water",
            Self::Light => "light",
            Self::Temperature => "temperature",
            Self::Humidity => "humidity",
            Self::Location => "location",
        };
        f.write_str(as_str)
    }
}

#[derive(Clone, Debug)]
struct CategoryRange {
    source: Range<u64>,
    destination: Range<u64>,
}

#[derive(Clone, Debug)]
struct SourceToDestination {
    source: Category,
    destination: Category,
    ranges: Vec<CategoryRange>,
}

impl SourceToDestination {
    fn destination_for(&self, num: u64) -> u64 {
        for range in &self.ranges {
            if range.source.contains(&num) {
                return range.destination.start + (num - range.source.start);
            }
        }
        num
    }
}

#[derive(Clone, Debug)]
struct Almanac {
    seeds: Vec<u64>,
    // using a BTreeMap here gives us sorted iteration
    // so we can iterate from the lowest to the highest category
    // just by using iter()
    src_to_dst_maps: BTreeMap<Category, SourceToDestination>,
}

impl Almanac {
    fn resolve_for(&self, seed: u64) -> Option<u64> {
        self.src_to_dst_maps
            .iter()
            .scan(seed, |num, (_, map)| {
                let destination_num = map.destination_for(*num);
                *num = destination_num;
                Some((map.destination, destination_num))
            })
            .find_map(|(c, num)| (c == Category::Location).then_some(num))
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    separated_pair(tag("seeds:"), space1, separated_list1(space1, u64))
        .map(|(_, seeds)| seeds)
        .parse(input)
}

fn parse_category(input: &str) -> IResult<&str, Category> {
    alt((
        tag("seed").map(|_| Category::Seed),
        tag("soil").map(|_| Category::Soil),
        tag("fertilizer").map(|_| Category::Fertilizer),
        tag("water").map(|_| Category::Water),
        tag("light").map(|_| Category::Light),
        tag("temperature").map(|_| Category::Temperature),
        tag("humidity").map(|_| Category::Humidity),
        tag("location").map(|_| Category::Location),
    ))
    .parse(input)
}

fn parse_src_to_dst(input: &str) -> IResult<&str, SourceToDestination> {
    separated_pair(
        tuple((
            separated_pair(parse_category, tag("-to-"), parse_category),
            tag(" map:"),
        )),
        multispace1,
        separated_list1(
            newline,
            separated_list1(space1, u64).map(|v| (v[0]..v[0] + v[2], v[1]..v[1] + v[2])),
        ),
    )
    .map(|(((src, dst), _), ranges)| {
        let ranges = ranges
            .into_iter()
            .map(|range| CategoryRange {
                source: range.1,
                destination: range.0,
            })
            .collect();
        SourceToDestination {
            source: src,
            destination: dst,
            ranges,
        }
    })
    .parse(input)
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, (seeds, src_to_dst_maps)) = separated_pair(
        parse_seeds,
        multispace1,
        many1(tuple((parse_src_to_dst, multispace1))).map(|v| {
            v.into_iter()
                .map(|(src_to_dst, _)| (src_to_dst.source, src_to_dst))
                .collect()
        }),
    )
    .parse(input)?;

    Ok((
        input,
        Almanac {
            seeds,
            src_to_dst_maps,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!("35", process(input)?);
        Ok(())
    }
}
