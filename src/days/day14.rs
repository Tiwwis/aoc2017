use crate::day10::knot_hash;
use crate::helpers;
use crate::helpers::{DayString, Solution};

type Squares = [[u8; 16]; 128];
type Pos = (i16, i16);
type Visit = [[bool; 128]; 128];

struct Disk {
    area: Squares,
}

impl Disk {
    fn new(area: Squares) -> Disk {
        Disk { area }
    }

    fn is_square(&self, index: Pos) -> bool {
        let (x, y) = index;
        if x > 127 || y > 127 || x < 0 || y < 0 {
            return false;
        };
        let (outer_x, inner_x) = (x / 8, x % 8);
        (self.area[y as usize][outer_x as usize] & (128 >> inner_x)) != 0
    }

    fn regions(&self) -> usize {
        let mut visit = [[false; 128]; 128];
        let mut regions = 0;

        for x in 0..128 {
            for y in 0..128 {
                if !visit[x as usize][y as usize] && self.is_square((x, y)) {
                    regions += 1;
                    self.dfs((x, y), &mut visit);
                }
            }
        }

        regions
    }

    fn dfs(&self, pos: Pos, visit: &mut Visit) {
        let (x, y) = pos;
        if !self.is_square(pos) || visit[x as usize][y as usize] {
            return;
        }
        //dbg!(pos);

        visit[x as usize][y as usize] = true;
        for nb_pos in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            self.dfs(nb_pos, visit)
        }
    }
}

fn get_squares(original: DayString) -> Squares {
    let mut result = [[0; 16]; 128];
    for (i, val) in result.iter_mut().enumerate() {
        let new_str = original.to_string() + "-" + &i.to_string();
        *val = knot_hash(new_str.as_bytes())
    }
    result
}

fn solve_part1(squares: &Squares) -> usize {
    squares
        .iter()
        .map(|arr| {
            arr.iter()
                .map(|num| num.count_ones() as usize)
                .sum::<usize>()
        })
        .sum()
}

fn solve_part2(area: Squares) -> usize {
    let disk = Disk::new(area);
    disk.regions()
}

fn solve_string(s: DayString) -> Solution {
    let squares = get_squares(s);
    let sol1 = solve_part1(&squares).to_string();
    let sol2 = solve_part2(squares).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(14))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = get_squares("flqrgnkx");
        assert_eq!(solve_part2(input), 1242);
    }
}
