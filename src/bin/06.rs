use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().collect_vec();
    let cols = &lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let ops = lines.last().unwrap().split_whitespace().collect_vec();

    let mut all = 0;
    for x in 0..cols[0].len() {
        let op_str = ops[x];
        let mut result = match op_str {
            "*" => 1,
            "+" => 0,
            _ => panic!(),
        };
        for y in 0..cols.len() {
            let num = cols[y][x];
            match op_str {
                "*" => result *= num,
                "+" => result += num,
                _ => panic!(),
            }
        }
        all += result
    }

    Some(all)
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
