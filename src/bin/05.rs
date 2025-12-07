use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Range {
    from: u64,
    to: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges_input, ingredients_input) = input.split("\n\n").collect_tuple().unwrap();
    let ranges = ranges_input
        .lines()
        .map(|line| {
            let (from, to) = line
                .split("-")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Range { from, to }
        })
        .collect_vec();

    let ingredients = ingredients_input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect_vec();

    let mut fresh_counts = 0;

    'outer: for ingredient in ingredients {
        for range in ranges.iter() {
            if ingredient >= range.from {
                if ingredient <= range.to {
                    fresh_counts += 1;
                    continue 'outer;
                }
            }
        }
    }

    Some(fresh_counts)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges_input, _) = input.split("\n\n").collect_tuple().unwrap();
    let ranges = ranges_input
        .lines()
        .map(|line| {
            let (from, to) = line
                .split("-")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Range { from, to }
        })
        .sorted_by(|a, b| Ord::cmp(&a.from, &b.from))
        .collect_vec();

    let mut curr_max = 0;
    let mut count = 0;
    ranges.iter().for_each(|range| {
        if range.to > curr_max {
            if curr_max >= range.from {
                count += range.to - curr_max;
            } else {
                count += range.to - range.from + 1;
            }
            curr_max = range.to;
        }
    });

    Some(count)
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
        assert_eq!(result, Some(14));
    }
}
