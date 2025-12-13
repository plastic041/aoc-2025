use std::char::ParseCharError;

use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};

advent_of_code::solution!(12);

#[derive(Debug)]
struct Region {
    x: usize,
    y: usize,
    shapes: Vec<u64>,
}

#[derive(Debug)]
struct Shape<'a> {
    id: u64,
    shape: &'a str,
    count: usize,
}

/// 5:
/// ###
/// .#.
/// ###
fn parse_shape(input: &str) -> Shape<'_> {
    let (id_str, shape) = input.split(":\n").collect_tuple().unwrap();

    Shape {
        id: id_str.parse().unwrap(),
        shape,
        count: shape.chars().filter(|c| *c == '#').count(),
    }
}

/// 4x4: 0 0 0 0 2 0
fn parse_region(input: &str) -> IResult<&str, Region> {
    let (input, s) = separated_pair(
        separated_pair(
            map(digit1, |s: &str| s.parse::<usize>().unwrap()),
            tag("x"),
            map(digit1, |s: &str| s.parse::<usize>().unwrap()),
        ),
        tag(": "),
        separated_list1(space1, map(digit1, |s: &str| s.parse::<u64>().unwrap())),
    )
    .parse(input)?;

    Ok((
        input,
        Region {
            x: s.0.0,
            y: s.0.1,
            shapes: s.1,
        },
    ))
}

fn can_fit(region: &Region, shapes: &[Shape]) -> bool {
    let count_sum: usize = region
        .shapes
        .iter()
        .enumerate()
        .map(|(id, shape_count)| shapes[id].count * *shape_count as usize)
        .sum();

    count_sum <= region.x * region.y
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut chunks = input.split("\n\n");
    let shapes = chunks.by_ref().take(6).map(parse_shape).collect_vec();
    let regions = chunks
        .last()
        .unwrap()
        .lines()
        .flat_map(parse_region)
        .map(|(_, r)| r)
        .collect_vec();

    let count = regions
        .iter()
        .filter(|region| can_fit(region, &shapes))
        .count();

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_shape() {
        let result = parse_shape(
            r"5:
###
.#.
###",
        );
        dbg!(result);
    }

    #[test]
    fn test_parse_region() {
        let result = parse_region("4x4: 0 0 0 0 2 0").unwrap();
        dbg!(result);
    }
}
