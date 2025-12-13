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

enum Operator {
    Product,
    Sum,
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect_vec();

    let mut sum: u64 = 0;

    let mut operator: Option<Operator> = None;

    let mut current_numbers: Vec<u64> = vec![];

    for x in 0..lines[0].len() {
        let mut is_empty = true;

        let mut current_num: Option<u64> = None;
        for y in 0..lines.len() {
            let char = lines[y].as_bytes()[x] as char;
            if char.is_whitespace() {
            } else {
                is_empty = false;
                if char.is_numeric() {
                    if let Some(cnum) = current_num {
                        current_num = Some(cnum * 10 + char.to_digit(10).unwrap() as u64);
                    } else {
                        current_num = Some(char.to_digit(10).unwrap() as u64);
                    }
                } else if char == '*' {
                    operator = Some(Operator::Product);
                } else if char == '+' {
                    operator = Some(Operator::Sum);
                }
            }
        }
        if let Some(cnum) = current_num {
            current_numbers.push(cnum);
        }

        if is_empty {
            if let Some(ref op) = operator {
                match op {
                    Operator::Product => sum += current_numbers.iter().product::<u64>(),
                    Operator::Sum => sum += current_numbers.iter().sum::<u64>(),
                }
            } else {
                panic!("has num but no operator");
            }

            operator = None;
            current_numbers.clear();
        }
    }

    if let Some(ref op) = operator {
        match op {
            Operator::Product => sum += current_numbers.iter().product::<u64>(),
            Operator::Sum => sum += current_numbers.iter().sum::<u64>(),
        }
    } else {
        panic!("has num but no operator");
    }

    Some(sum)
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
        assert_eq!(result, Some(3263827));
    }
}
