use itertools::Itertools;

advent_of_code::solution!(2);

// fn parse_pair(input: &str) -> IResult<&str, (u32, u32)> {
//     let (input, p) = map(
//         separated_pair(digit1(), tag("-"), digit1()),
//         |(a, b): (&str, &str)| {
//             (
//                 a.parse::<u32>().expect("number error"),
//                 b.parse::<u32>().expect("number"),
//             )
//         },
//     )
//     .parse(input)?;

//     Ok((input, p))
// }

// fn parse_line(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
//     let (input, v) = separated_list1(tag(","), parse_pair).parse(input)?;

//     Ok((input, v))
// }

fn is_invalid_id(num: u64) -> bool {
    let numstr = num.to_string();
    let half_len = numstr.len() / 2;
    &numstr[..half_len] == &numstr[half_len..]
}

fn check(input: &str, size: usize) -> bool {
    let len = input.len();
    if len % size != 0 {
        return false;
    }

    let bytes = input.as_bytes();

    let chunk_count = len / size;

    for char_index in 0..size {
        for chunk_index in 0..chunk_count {
            let first = bytes[char_index];

            let real_char_index = chunk_index * size + char_index;
            if bytes[real_char_index] != first {
                return false;
            }
        }
    }

    return true;
}

fn is_invalid_id2(num: u64) -> bool {
    let numstr = num.to_string();
    let len = numstr.len();
    let half_len = len / 2;

    for l in 1..=half_len {
        if len % l == 0 {
            let checked = check(&numstr, l);
            if checked {
                return true;
            }
            // let chunks: HashSet<&str> = numstr
            //     .as_bytes()
            //     .chunks(l)
            //     .map(|chunk| std::str::from_utf8(chunk).unwrap())
            //     .collect();
            // if chunks.len() == 1 {
            //     return true;
            // }
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let id_group = input
        .split(",")
        .map(|pat| {
            pat.split("-")
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple::<(u64, u64)>()
                .unwrap()
        })
        .collect_vec();

    let mut invalids = 0;
    for group in id_group {
        for i in group.0..=group.1 {
            if is_invalid_id(i) {
                invalids += i;
            }
        }
    }

    Some(invalids)
}

pub fn part_two(input: &str) -> Option<u64> {
    let id_group = input
        .split(",")
        .map(|pat| {
            pat.split("-")
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple::<(u64, u64)>()
                .unwrap()
        })
        .collect_vec();

    let mut invalids = 0;
    for group in id_group {
        for i in group.0..=group.1 {
            if is_invalid_id2(i) {
                invalids += i;
            }
        }
    }

    Some(invalids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    fn test_check() {
        let result = check("123123", 3);
        assert!(result);

        let result2 = check("123123", 2);
        assert!(!result2);
    }
}
