use std::collections::HashMap;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
};
use pathfinding::prelude::count_paths;

advent_of_code::solution!(11);

fn parse(input: &str) -> IResult<&str, HashMap<&str, Vec<&str>>> {
    let (input, graph) = separated_list1(
        line_ending,
        separated_pair(alpha1, tag(": "), separated_list1(space1, alpha1)),
    )
    .parse(input)?;

    Ok((input, graph.into_iter().collect()))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, graph) = parse(input).unwrap();

    let path_counts = count_paths(
        "you",
        |s| graph.get(*s).unwrap_or(&vec![]).clone(),
        |c| *c == "out",
    );

    Some(path_counts)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, graph) = parse(input).unwrap();

    let counts_svr_fft = count_paths(
        "svr",
        |s| graph.get(*s).unwrap_or(&vec![]).clone(),
        |c| *c == "fft",
    );
    let counts_fft_dac = count_paths(
        "fft",
        |s| graph.get(*s).unwrap_or(&vec![]).clone(),
        |c| *c == "dac",
    );
    let counts_dac_out = count_paths(
        "dac",
        |s| graph.get(*s).unwrap_or(&vec![]).clone(),
        |c| *c == "out",
    );

    Some(counts_svr_fft * counts_fft_dac * counts_dac_out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(5));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
