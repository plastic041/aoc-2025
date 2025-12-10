use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let tiles = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple::<(u64, u64)>()
                .unwrap()
        })
        .collect_vec();

    let mut max = 0;
    tiles.into_iter().tuple_combinations().for_each(|(a, b)| {
        let width = a.0.abs_diff(b.0) + 1;
        let height = a.1.abs_diff(b.1) + 1;
        let size = width * height;
        if max < size {
            max = size;
        }
    });

    Some(max)
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
