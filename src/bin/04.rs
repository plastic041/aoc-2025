use itertools::Itertools;

advent_of_code::solution!(4);

#[derive(Debug)]
enum Cell {
    Empty,
    Roll,
}

fn get_neighbors(x: isize, y: isize) -> Vec<(isize, isize)> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .map(|(dx, dy)| (x + dx, y + dy))
    .into_iter()
    .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Roll,
                    _ => panic!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut counts = 0;
    for y in 0..rows {
        for x in 0..cols {
            let element = grid.get(y).and_then(|line| line.get(x)).unwrap();
            if let Cell::Roll = element {
                let neighbors = get_neighbors(x as isize, y as isize);
                let neighbors_count = neighbors
                    .into_iter()
                    .map(|(nx, ny)| grid.get(ny as usize).and_then(|line| line.get(nx as usize)))
                    .flatten()
                    .filter(|cell| matches!(cell, Cell::Roll))
                    .count();

                if neighbors_count < 4 {
                    counts += 1;
                }
            }
        }
    }

    Some(counts)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Roll,
                    _ => panic!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut counts = 0;
    let mut to_remove: Vec<(usize, usize)> = vec![];

    loop {
        for y in 0..rows {
            for x in 0..cols {
                let element = grid.get(y).and_then(|line| line.get(x)).unwrap();
                if let Cell::Roll = element {
                    let neighbors = get_neighbors(x as isize, y as isize);
                    let neighbors_count = neighbors
                        .into_iter()
                        .map(|(nx, ny)| {
                            grid.get(ny as usize).and_then(|line| line.get(nx as usize))
                        })
                        .flatten()
                        .filter(|cell| matches!(cell, Cell::Roll))
                        .count();

                    if neighbors_count < 4 {
                        to_remove.push((x, y))
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break;
        } else {
            counts += to_remove.len();
            to_remove
                .iter()
                .for_each(|(x, y)| grid[*y][*x] = Cell::Empty);
            to_remove.clear();
        }
    }

    Some(counts as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
