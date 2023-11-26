use crate::helpers;
use crate::helpers::{DayString, Solution};

use std::collections::hash_map;
use std::str::FromStr;

use ndarray::{Array2, ArrayView2, ArrayViewMut2, Axis, s};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Light {
    On,
    Off,
}

impl Light {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Light::On),
            '.' => Some(Light::Off),
            _ => None,
        }
    }
}

type Grid = Array2<Light>;
type Rules = hash_map::HashMap<Grid, Grid>;
type Rule = (Grid, Grid);

fn parse_rule(s: &str) -> Rule {
    fn parse_grid(s: &str) -> Grid {
        let n = s.split('/').next().unwrap().len();
        let v = s.chars().filter_map(Light::from_char).collect();
        Array2::from_shape_vec([n, n], v).unwrap()
    }

    let [left, right]: [&str; 2] = s
        .splitn(2, " => ")
        .collect::<Vec<&str>>()
        .try_into()
        .expect("Rule should only have 2 components!");

    (parse_grid(left), parse_grid(right))
}

fn parse_input(s: DayString) -> Rules {
    s.lines().map(parse_rule).collect()
}

fn expand_grid(grid: &Grid, rules: &Rules) -> Grid {
    let n = grid.len_of(Axis(0));
    let (new_n, old_sq, new_sq) = if n % 2 == 0 {
        (n / 2 * 3, 2, 3)
    } else {
        (n / 3 * 4, 2, 3)
    };

    let mut new_grid = Array2::from_elem([new_n; 2], Light::Off);
    for i in 0..n/old_sq {
        for j in 0..n/old_sq {
            let old_si = old_sq*i;
            let old_ei = old_si + old_sq;
            let old_sj = old_sq*j;
            let old_ej = old_sj + old_sq;

            let old_slice = grid.slice(s![old_si..old_ei,old_sj..old_ej]);
            let new_slice = rules.get(&old_slice.to_owned()).unwrap();
            let mut target_slice = new_grid.slice_mut(s![new_sq*i..new_sq*(i+1), new_sq*j..new_sq*(j+1)]);
            target_slice.assign(new_slice);
        }
    }

    new_grid
}

fn solve_part1(input: &Rules) -> usize {
    unimplemented!();
}

fn solve_part2(input: &Rules) -> usize {
    unimplemented!();
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let sol1 = solve_part1(&parsed).to_string();
    let sol2 = solve_part2(&parsed).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(21))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(helpers::read_example("xxx"));
        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(helpers::read_example("xxx"));
        assert_eq!(solve_part2(&input), 3);
    }
}
