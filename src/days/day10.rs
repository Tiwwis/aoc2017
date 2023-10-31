use crate::helpers;
use crate::helpers::{DayString, Solution};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct StringCircle {
    list: [u8; 256],
}

impl StringCircle {
    fn new() -> Self {
        let mut list: [u8; 256] = [0; 256];
        for (i, val) in list.iter_mut().enumerate() {
            *val = i as u8
        }
        Self { list }
    }

    fn reverse_slice(&mut self, start: usize, len: usize) {
        let end = start + len - 1;
        let half = len / 2;
        for k in 0..half {
            let i = start + k;
            let j = end - k;
            (self[i], self[j]) = (self[j], self[i]);
        }
    }
}

impl Index<usize> for StringCircle {
    type Output = u8;

    fn index(&self, n: usize) -> &Self::Output {
        let len = self.list.len();
        &self.list[n % len]
    }
}

impl IndexMut<usize> for StringCircle {
    fn index_mut(&mut self, n: usize) -> &mut Self::Output {
        let len = self.list.len();
        &mut self.list[n % len]
    }
}

fn parse_numbers(s: DayString) -> Vec<u8> {
    s.split(',').filter_map(|x| x.parse().ok()).collect()
}

fn solve_part1(lengths: &[u8]) -> usize {
    let mut circ = StringCircle::new();

    let mut pos = 0;

    for (skip, len) in lengths.iter().enumerate() {
        let len = *len as usize;
        circ.reverse_slice(pos, len);
        pos += len + skip;
    }
    circ[0] as usize * circ[1] as usize
}

fn dense_hash(table: &[u8; 256]) -> [u8; 16] {
    let mut out: [u8; 16] = [0; 16];

    for (i, val) in table.iter().enumerate() {
        out[i / 16] ^= val
    }

    out
}

fn as_hex(dense: &[u8; 16]) -> String {
    let mut out = "".to_string();
    for x in dense {
        out.push_str(&format!("{:02x?}", x))
    }
    out
}

pub fn knot_hash(bytes: &[u8]) -> [u8; 16] {
    let mut new_lengths = Vec::new();

    new_lengths.extend(bytes);
    new_lengths.extend([17, 31, 73, 47, 23]);

    let lengths = new_lengths;

    let mut circ = StringCircle::new();

    let mut skip = 0;
    let mut pos = 0;

    for _ in 0..64 {
        for len in &lengths {
            let len = *len as usize;
            circ.reverse_slice(pos, len);
            pos += len + skip;
            skip += 1;
        }
    }

    dense_hash(&circ.list)
}

pub fn knot_hash_str(string: &str) -> String {
    as_hex(&knot_hash(string.as_bytes()))
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_numbers(s);
    let sol1 = solve_part1(&parsed).to_string();
    let sol2 = knot_hash_str(s);
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(knot_hash_str(&input), "a2582a3a0e66e6e86e3812dcb672a272");
    }
}
