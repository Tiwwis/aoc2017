use crate::helpers;
use crate::helpers::{DayString, Solution};
use ndarray;

type Path = ndarray::Array2<Field>;
type Pos = [usize; 2];
type Dir = [isize; 2];

fn rot_left([x,y]:Dir) -> Dir {
    [-y, x]
}

fn rot_right([x,y]:Dir) -> Dir {
    [y, -x]
}

fn go([x,y]:Pos, [xx, yy]:Dir) -> Pos {
    [(x as isize + xx) as usize, (y as isize + yy) as usize]
}

#[derive(Debug)]
struct Walker {
    pos: Pos,
    dir: Dir,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Field {
    Empty,
    Vert,
    Hor,
    Cross,
    Letter(char),
}

#[derive(Debug)]
struct ParseFieldError;

impl Walker {
    fn step(&mut self) {
        self.pos = go(self.pos, self.dir);
    }

    fn walk(&mut self, path:&Path, res: &mut Vec<char>) -> bool {
        let here = path[self.pos];
        match here {
            Field::Vert => { self.step(); true },
            Field::Hor => {self.step(); true },
            Field::Cross => {
                let left_dir = rot_left(self.dir);
                let right_dir = rot_right(self.dir);
                let left_field = go(self.pos, left_dir);
                match path[left_field] {
                    Field::Empty => {
                        self.dir = right_dir;
                        self.step()
                    }
                    _ => {
                        self.dir = left_dir;
                        self.step()
                    } 
                }
                true
            }
            Field::Letter(a) => {
                res.push(a);
                self.step();
                true
            }
            Field::Empty => false,
        }
    }

    fn start(path:&Path) -> Walker {
        let (i, _) = path.row(0).iter().enumerate().find(|(_, f)| matches!(f, Field::Vert)).expect("No start found!");
        Walker { pos: [0, i], dir: [1, 0] }
    }
}

impl TryInto<Field> for char {
    type Error = ParseFieldError;

    fn try_into(self) -> Result<Field, Self::Error> {
        match self {
            ' ' => Ok(Field::Empty),
            '|' => Ok(Field::Vert),
            '-' => Ok(Field::Hor),
            '+' => Ok(Field::Cross),
            v if v.is_ascii_uppercase() => Ok(Field::Letter(v)),
            _ => Err(ParseFieldError),
        }
    }
}

fn right_pad(s: &str, n: usize, symb: char) -> String {
    let new = s.to_string();
    let to_add = n.saturating_sub(new.len());
    new + &symb.to_string().repeat(to_add)
}

fn parse_input(s: DayString) -> Path {
    let line_len: usize = s
        .lines()
        .map(str::len)
        .max()
        .expect("Lines should not be empty!");

    let v: Vec<Field> = s
        .lines()
        .chain(Some("").into_iter())
        .flat_map(|line| {
            right_pad(line, line_len, ' ')
                .chars()
                .map(|c| c.try_into().unwrap())
                .collect::<Vec<Field>>()
        })
        .collect();

    let n_rows = v.len()/line_len;
    ndarray::Array::from_shape_vec([n_rows, line_len], v).unwrap()
}

fn solve_day(input: &Path) -> (String, usize) {
    let mut walker = Walker::start(input);
    let mut res = Vec::new();
    let mut count = 0;
    loop {
        let walking = walker.walk(input, &mut res);
        count += 1;
        if !walking { break (res.iter().collect(), count - 1) };
    }
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let (sol1, sol2) = solve_day(&parsed);
    let sol1 = sol1.to_string();
    let sol2 = sol2.to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(19))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input =parse_input(helpers::read_example("19"));
        assert_eq!(input[[0,5]], Field::Vert);
    }
    #[test]
    fn test_part1() {
        let input = parse_input(helpers::read_example("19"));
        let (path, _) = solve_day(&input);
        assert_eq!(path, "ABCDEF");
    }

    #[test]
    fn test_part2() {
        let input = parse_input(helpers::read_example("19"));
        let (_, n) = solve_day(&input);
        assert_eq!(n, 38);
    }
}
