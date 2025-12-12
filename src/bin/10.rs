// https://topaz.github.io/paste/#XQAAAQApCwAAAAAAAAA0m0pnuFI8dAwofC54jwTzIpB+B2MAKxKwYqOArdyIoGaSc154fTSGY3sLTIr2jpg5FEmYwkoe/7LJnxuIeiBr1eulSXGY45GWZaiS+HbcNDMgN2JU0g4h052bZTqc8JyLqoTG2mUjhvleITbEsMO5RIRM44wINvAQZn9EZPjTU4mdtofegcYwAgF75P9iRaLrU7ey+b+GmN6OPG1jhTTDLszZwqQXcMkoW7azKDPet1nIgKO/7XkVYHEJXx4HxupVOj1sdjBGlryO2IDwL0hiGp30sDfApQ3Knl1Nv68+iigjPnWFH3bSL223zqmZqGi432v6TnErCpmXQuuh5YmN8rf/FVgJv7JrsiRoSkfuTaw2+4JsG4V3zX2hqpwkM7/BbXEwso7v4ai9j0zawWI4D1DYe1jTwm0SSnBiOFde8CUwQgr0I+b/ewlYRCAt6JRzCaIaprQHzySUxG3oIHkVrgJIJ9CTIVQQ1swul8T/0NVJXDBtVLwc0NI136Cyo6FXBCZK29Wh7gzHm8knmEi5Ophze6NFBRUmH0Ck85TicLy8kpaW6Ey5WAm8G9qlAzyItdujOHjphu8qQDqZoGlAjmdGhKEFyolFcKzvTqnDWXSI6JFDSulvrwh91fANr1RVewZ02/+XnwC0kCJV+fr4qoH5p3rSllFV29O0P1LSCzu8uAZNaOQUHa6BcKhVzZfRvSarVU6CVWzgCKBYjuMXV00F+hJK5sLEt5US6GQHcvJehcdfFWY++PvrCpjncectJLS3wKC5ed7tu/vb9Koky5+gH86m47aVlIBHwEOhMNydHJHkUXnKqqIdIKl8eGGEmYDe8QER7MAbhyoTokl5DvsBU6h6aJiD4bZF7ZJ2dukfJw/EulhZxE8yWf/n+WiF

advent_of_code::solution!(10);

use itertools::Itertools;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, digit1, multispace1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded},
};
use z3::ast::Int;
use z3::{Optimize, SatResult};

#[derive(Debug, Clone)]
pub struct Machine {
    pub indicators: Vec<bool>,
    pub buttons: Vec<Vec<usize>>,
}

fn indicator(input: &str) -> IResult<&str, bool> {
    alt((map(char('.'), |_| false), map(char('#'), |_| true))).parse(input)
}

fn indicators(input: &str) -> IResult<&str, Vec<bool>> {
    delimited(char('['), nom::multi::many1(indicator), char(']')).parse(input)
}

fn usize_parser(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>()).parse(input)
}

fn button(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        char('('),
        separated_list1(char(','), usize_parser),
        char(')'),
    )
    .parse(input)
}

fn buttons(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(multispace1, button).parse(input)
}

pub fn parse_machine(input: &str) -> IResult<&str, Machine> {
    map(
        (indicators, preceded(multispace1, buttons)),
        |(indicators, buttons)| Machine {
            indicators,
            buttons,
        },
    )
    .parse(input)
}

impl Machine {
    fn find_min_presses(&self) -> Result<u64, String> {
        let optimizer = Optimize::new();

        let mut button_vars = Vec::new();
        for i in 0..self.buttons.len() {
            let button = Int::new_const(format!("button_{}", i));
            button_vars.push(button.clone());
            optimizer.assert(&button.ge(&Int::from_i64(0)));
        }

        for i in 0..self.indicators.len() {
            let target = if self.indicators[i] { 1 } else { 0 };
            let mut buttons = Vec::new();

            for j in 0..self.buttons.len() {
                if self.buttons[j].contains(&i) {
                    buttons.push(&button_vars[j]);
                }
            }

            if !buttons.is_empty() {
                let sum = buttons
                    .iter()
                    .skip(1)
                    .fold(buttons[0].clone(), |acc, button| acc + (*button).clone());
                let modulo = sum.modulo(&Int::from_i64(2));
                optimizer.assert(&modulo.eq(&Int::from_i64(target)));
            }
        }

        assert!(!button_vars.is_empty());

        let total = button_vars
            .iter()
            .skip(1)
            .fold(button_vars[0].clone(), |acc, button| acc + button.clone());
        optimizer.minimize(&total);

        match optimizer.check(&[]) {
            SatResult::Sat => {
                let model = optimizer.get_model().unwrap();
                let result = button_vars
                    .iter()
                    .map(|button| model.eval(button, true).unwrap().as_u64().unwrap())
                    .sum();
                Ok(result)
            }
            _ => Err("No solution".to_string()),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = input
        .lines()
        .flat_map(parse_machine)
        .map(|(_, m)| m)
        .collect_vec();

    Some(machines.iter().flat_map(Machine::find_min_presses).sum())
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
