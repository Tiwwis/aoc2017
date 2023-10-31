use crate::helpers;
use crate::helpers::{DayString, Solution};

type Path = Vec<HexStep>;

enum HexStep {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

struct HexCoord(i32, i32);

fn parse_input(s: DayString) -> Path {
    s.split(',')
        .map(|s| match s {
            "n" => HexStep::N,
            "ne" => HexStep::NE,
            "se" => HexStep::SE,
            "s" => HexStep::S,
            "sw" => HexStep::SW,
            "nw" => HexStep::NW,
            _ => panic!("Unknown step"),
        })
        .collect()
}

impl HexCoord {
    fn new() -> Self {
        HexCoord(0, 0)
    }

    fn step(&mut self, step: &HexStep) {
        match step {
            HexStep::N => self.1 += 1,
            HexStep::NE => {
                self.0 += 1;
                self.1 += 1
            }
            HexStep::SE => self.0 += 1,
            HexStep::S => self.1 -= 1,
            HexStep::SW => {
                self.0 -= 1;
                self.1 -= 1
            }
            HexStep::NW => self.0 -= 1,
        }
    }

    fn hex_norm(&self) -> i32 {
        let x = self.0;
        let y = self.1;
        if x.signum() != y.signum() {
            x.abs() + y.abs()
        } else {
            x.abs().min(y.abs())
        }
    }
}
fn solve_day(steps: &Path) -> (i32, i32) {
    let mut start = HexCoord::new();
    let max_d = steps
        .iter()
        .map(|step| {
            start.step(step);
            start.hex_norm()
        })
        .max()
        .unwrap_or_default();
    (start.hex_norm(), max_d)
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let (sol1, sol2) = solve_day(&parsed);
    let sol1 = sol1.to_string();
    let sol2 = sol2.to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(11))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let i1 = parse_input("ne,ne,ne");
        let i2 = parse_input("ne,ne,sw,sw");
        let i3 = parse_input("ne,ne,s,s");
        let i4 = parse_input("se,s,se,sw,sw");

        assert_eq!(solve_day(&i1).0, 3);
        assert_eq!(solve_day(&i2).0, 0);
        assert_eq!(solve_day(&i3).0, 2);
        assert_eq!(solve_day(&i4).0, 3);
    }
}
