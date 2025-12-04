use itertools::Itertools;

advent_of_code::solution!(3);

fn find_largest_battery(input: &str, digits: usize) -> u64 {
    let bytes = input.as_bytes();

    let mut numbers = vec![];

    let mut last_index: i32 = -1;

    for digit in 1..=digits {
        let mut largest = 0;

        for i in (last_index + 1) as usize..bytes.len() - digits + digit {
            let num = bytes[i];
            if largest < num {
                largest = num;
                last_index = i as i32;
            }
        }
        numbers.push(largest);
    }

    numbers
        .iter()
        .map(|num| *num as char)
        .join("")
        .parse::<u64>()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let banks = input.lines();
    let batteries = banks.map(|bank| find_largest_battery(bank, 2)).sum();

    Some(batteries)
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = input.lines();
    let batteries = banks.map(|bank| find_largest_battery(bank, 12)).sum();

    Some(batteries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }

    #[test]
    fn test_largest_battery() {
        let result = find_largest_battery("234234234234278", 2);
        assert_eq!(result, 78);

        let result = find_largest_battery("987654321111111", 12);
        assert_eq!(result, 987654321111);
    }
}
