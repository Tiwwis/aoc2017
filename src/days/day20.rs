use std::collections::hash_map;
use std::ops::Add;

use crate::helpers;
use crate::helpers::{DayString, Solution};

use nom::bytes::complete::tag;
use nom::{
    character::{complete::char, complete::i64},
    combinator::{map_opt, opt},
    multi::count,
    sequence::{delimited, preceded, terminated, tuple},
};
use nom::{IResult, Parser};

type Input = Vec<Particle>;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector {
    fn norm(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Debug, Hash, Clone)]
struct Particle {
    p: Vector,
    v: Vector,
    a: Vector,
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Particle {
    fn step(&self) -> Particle {
        let v = self.v + self.a;
        let p = self.p + v;
        let a = self.a;
        Particle { p, a, v }
    }
}

fn vector_parser(s: &str) -> IResult<&str, Vector> {
    let num_parser = terminated(i64, opt(char(',')));
    let done = delimited(char('<'), count(num_parser, 3), char('>'));
    map_opt(done, |v| match &v[..] {
        &[x, y, z] => Some(Vector { x, y, z }),
        _ => None,
    })(s)
}

fn particle_parser(s: &str) -> IResult<&str, Particle> {
    let p_parse = preceded(tag("p="), vector_parser);
    let v_parse = preceded(tag(", v="), vector_parser);
    let a_parse = preceded(tag(", a="), vector_parser);
    tuple((p_parse, v_parse, a_parse))
        .map(|(p, v, a)| Particle { p, v, a })
        .parse(s)
}

fn simulate_step(particles: &[Particle]) -> Vec<Particle> {
    let next: Vec<Particle> = particles.iter().map(Particle::step).collect();
    let mut dupli: Vec<bool> = vec![true; next.len()];
    let mut positions = hash_map::HashMap::with_capacity(next.len());
    for (i, particle) in next.iter().enumerate() {
        let entry = positions.entry(particle.p);
        match entry {
            hash_map::Entry::Occupied(e) => {
                dupli[*e.get()] = false;
                dupli[i] = false;
            }
            hash_map::Entry::Vacant(e) => {
                e.insert(i);
            }
        }
    }

    next.into_iter()
        .zip(dupli.iter())
        .filter_map(|(p, dup)| dup.then_some(p))
        .collect()
}

fn parse_input(s: DayString) -> Input {
    s.lines()
        .map(|line| particle_parser.parse(line).unwrap().1)
        .collect()
}

fn solve_part1(input: &Input) -> usize {
    input
        .iter()
        .enumerate()
        .min_by(|(_, v1), (_, v2)| v1.a.norm().cmp(&v2.a.norm()))
        .unwrap()
        .0
}

fn solve_part2(input: &Input) -> usize {
    let mut particles = (*input).clone();
    let mut len = usize::MAX;
    let mut count = 0;
    loop {
        let cur_len = particles.len();
        if len == cur_len {
            count += 1;
        } else {
            count = 0;
            len = cur_len
        };
        if count > 10 {
            break cur_len
        }
        particles = simulate_step(&particles);
    }
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let sol1 = solve_part1(&parsed).to_string();
    let sol2 = solve_part2(&parsed).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(20))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(helpers::read_example("20"));
        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(helpers::read_example("xxx"));
        assert_eq!(solve_part2(&input), 3);
    }
}
