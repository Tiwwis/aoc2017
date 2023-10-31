use crate::helpers;
use crate::helpers::{DayString, Solution};

struct Circular {
    elements: Vec<CircNode>,
}

type CircIdx = usize;

#[derive(Clone, Copy, Debug)]
struct CircNode {
    value: u32,
    next: CircIdx,
}

impl CircNode {
    fn new(value: u32, next: CircIdx) -> Self {
        Self { value, next }
    }
}

impl Circular {
    fn new() -> Circular {
        Circular {
            elements: vec![CircNode::new(0, 0)],
        }
    }

    fn next(&self, idx: CircIdx) -> CircIdx {
        self.elements[idx].next
    }

    fn insert_after(&mut self, idx: CircIdx, value: u32) -> CircIdx {
        let elements = &mut self.elements;
        let len = elements.len();

        let this = &mut elements[idx];
        let new_node = CircNode::new(value, this.next);
        this.next = len;

        self.elements.push(new_node);
        len
    }

    fn spin_lock(&mut self, n: usize, n_iter: u32) -> u32 {
        let mut idx = Some(0);
        for i in 1..=n_iter {
            idx = std::iter::successors(idx, |&pt| Some(self.next(pt))).nth(n);
            idx = idx.map(|idx| self.insert_after(idx, i));
        }
        let idx = self.next(idx.unwrap());
        self.elements[idx].value
    }
}

fn parse_input(s: DayString) -> usize {
    s.parse().unwrap()
}

fn solve_part1(n: usize) -> u32 {
    let mut l = Circular::new();
    l.spin_lock(n, 2017)
}

fn solve_part2(n: usize) -> usize {
    let mut rel_to = 0;
    let mut result = 0;
    let mut i = 1;

    while i < 50_000_000 {
        let mut n_iters = (i - rel_to)/n;
        let remain = (i - rel_to)%n;
        if remain > 0 {n_iters += 1};

        let n_iter_before = n_iters - 1;
        rel_to = rel_to + n_iter_before*(n+1);

        i += n_iter_before;
        rel_to = (rel_to + n)%i + 1;

        if rel_to == 1 {
            result = i
        };

        i += 1;
    }
    result
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let sol1 = solve_part1(parsed).to_string();
    let sol2 = solve_part2(parsed).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(17))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = 3;
        assert_eq!(solve_part1(input), 638);
    }
}
