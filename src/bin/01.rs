advent_of_code::solution!(1);

use itertools::Itertools;
use nom::{
    IResult, Parser,
    character::complete::{digit1, one_of},
    combinator::{map, map_res},
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
// change this to - or + to possible performance gain

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    value: i32,
}

fn parse_rotation(input: &str) -> IResult<&str, Rotation> {
    let (input, direction) = map(one_of("LR"), |c| match c {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!(),
    })
    .parse(input)?;
    let (input, value) = map_res(digit1, |s: &str| s.parse::<i32>()).parse(input)?;

    Ok((input, Rotation { direction, value }))
}

pub fn part_one(input: &str) -> Option<i32> {
    let sequence = input
        .lines()
        .map(parse_rotation)
        .map_ok(|rot| rot.1)
        .flatten()
        .collect_vec();

    let mut lock = 50;
    let mut zero_count = 0;

    for rotation in sequence {
        let next = match rotation.direction {
            Direction::Left => -rotation.value,
            Direction::Right => rotation.value,
        };
        lock = (lock + next).rem_euclid(100);
        if lock == 0 {
            zero_count += 1;
        }
    }

    Some(zero_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sequence = input
        .lines()
        .map(parse_rotation)
        .map_ok(|rot| rot.1)
        .flatten()
        .collect_vec();

    let mut lock = 50;
    let mut zero_count = 0;

    for rotation in sequence {
        let change = match rotation.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };
        for _ in 0..rotation.value {
            lock = ((lock + change) as i32).rem_euclid(100);
            // not the smartest way
            if lock == 0 {
                zero_count += 1;
            }
        }
    }

    Some(zero_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
