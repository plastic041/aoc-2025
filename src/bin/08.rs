use std::collections::{BTreeMap, HashSet};

use itertools::Itertools;
use ordered_float::{Float, OrderedFloat, Pow};

advent_of_code::solution!(8);

fn dist(
    a: &(OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>),
    b: &(OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>),
) -> OrderedFloat<f32> {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let dz = b.2 - a.2;

    return (dx.pow(2.) + dy.pow(2.) + dz.pow(2.)).sqrt();
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut vectors: Vec<(OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>)> = vec![];
    let mut map = BTreeMap::new();

    input.lines().for_each(|line| {
        let (x, y, z) = line
            .split(",")
            .map(|s| s.parse::<OrderedFloat<f32>>().unwrap())
            .collect_tuple()
            .unwrap();
        vectors.push((x, y, z));
    });

    for (a, b) in vectors.iter().tuple_combinations() {
        let dist = dist(a, b);
        map.insert(dist, (a, b));
    }

    let mut circuits: Vec<HashSet<&(OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>)>> =
        vec![];
    let mut count = 0;

    for (_, (a, b)) in map {
        if count == 10 {
            break;
        }

        let mut should_push = true;
        for circuit in circuits.iter_mut() {
            let contains_a = circuit.contains(a);
            let contains_b = circuit.contains(b);
            match (contains_a, contains_b) {
                (true, true) => {}
                (true, false) => {
                    circuit.insert(b);
                    should_push = false;
                    break;
                }
                (false, true) => {
                    circuit.insert(a);
                    should_push = false;
                    break;
                }
                (false, false) => {}
            }
        }
        if should_push {
            println!("{:?}, {:?} {:?}", circuits, a.0, b.0);
            let mut set = HashSet::new();
            set.insert(a);
            set.insert(b);
            circuits.push(set)
        }

        count += 1;
    }

    for circuit in circuits.iter() {
        println!("{:?}", circuit);
    }

    for circuit in circuits {
        println!("{}", circuit.len());
    }

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
}
