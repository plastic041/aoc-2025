use itertools::Itertools;

advent_of_code::solution!(3);

fn find_largest_battery(input: &str) -> u8 {
    let bytes = input.as_bytes();

    let mut tenth_index = 0;
    let mut tenth_largest = 0;
    for (i, num) in bytes[..bytes.len() - 1].iter().enumerate() {
        if tenth_largest < *num {
            tenth_largest = *num;
            tenth_index = i;
        }
    }

    let tenth = bytes[tenth_index] as char;

    let mut oneth_index = 0;
    let mut oneth_largest = 0;
    for (i, num) in bytes[tenth_index + 1..].iter().enumerate() {
        if oneth_largest < *num {
            oneth_largest = *num;
            oneth_index = i + tenth_index + 1;
        }
    }
    let oneth = bytes[oneth_index] as char;

    let s = format!("{}{}", tenth, oneth);

    s.parse().unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let banks = input.lines();
    let batteries = banks.map(|bank| find_largest_battery(bank) as u64).sum();

    Some(batteries)
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_largest_battery() {
        let result = find_largest_battery("987654321111111");
        assert_eq!(result, 98);
    }
}
