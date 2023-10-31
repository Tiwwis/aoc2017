use crate::helpers;
use crate::helpers::{DayString, Solution};

use std::collections::hash_map::{
    Entry::{Occupied, Vacant},
    HashMap,
};

type Memory = Vec<u8>;
type Visited = HashMap<Memory, usize>;

fn parse_input(s: DayString) -> Memory {
    let mut mem: Memory = s
        .trim()
        .split(char::is_whitespace)
        .flat_map(str::parse)
        .collect();
    mem.shrink_to_fit();
    mem
}

fn redistribute(mem: &mut Memory) {
    let max_ind = mem
        .iter()
        .copied()
        .enumerate()
        .max_by_key(|&(i, n)| (n, -(i as i8)))
        .unwrap()
        .0;
    let mem_len = mem.len();
    let n = mem_len as u8;
    let k = mem[max_ind];

    let add = k / n;
    let rem = (k % n) as usize;

    mem[max_ind] = 0;

    (0..mem.len()).for_each(|i| mem[i] += add);
    (max_ind + 1..max_ind + 1 + rem).for_each(|i| mem[i % mem_len] += 1);
}

fn solve_day(input: &Memory) -> (usize, usize) {
    let mut visited = Visited::new();
    let mut mem = input.clone();
    let mut counter = 0;
    let time = loop {
        match visited.entry(mem.clone()) {
            Vacant(entry) => entry.insert(counter),
            Occupied(entry) => break *entry.get(),
        };
        redistribute(&mut mem);
        counter += 1;
    };
    (counter, counter - time)
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let (sol1, sol2) = solve_day(&parsed);
    [sol1.to_string(), sol2.to_string()]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(6))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(helpers::read_example("06"));
        assert_eq!(solve_day(&input).0, 5);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(helpers::read_example("06"));
        assert_eq!(solve_day(&input).1, 4);
    }
}
