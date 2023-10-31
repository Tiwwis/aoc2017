use std::str::FromStr;

use crate::helpers;
use crate::helpers::{DayString, Solution};

type Dance = Vec<Move>;

#[derive(Debug, Clone, Copy)]
enum Move {
    Spin(u8),
    Exchange(u8, u8),
    Partner(char, char)
}

#[derive(Debug)]
struct Dancers {
    dancers: Vec<char>
}

// abcdefg - gabcdef
impl Dancers {
    fn new() -> Self { Dancers { dancers: "abcdefghijklmnop".chars().collect() } }
    
    fn dance_move(&mut self, mov: &Move) {
        match mov {
            Move::Spin(x) => {
                let n = self.dancers.len();
                let x = *x as usize;
                let mut new_dancers:Vec<char> = Vec::new();
                new_dancers.extend(&self.dancers[n-x..n]);
                new_dancers.extend(&self.dancers[0..n-x]);
                self.dancers = new_dancers;
            },
            Move::Exchange(x, y) => {
                let x = *x as usize;
                let y = *y as usize;
                (self.dancers[x], self.dancers[y]) = (self.dancers[y], self.dancers[x]);
            },
            Move::Partner(a, b) => {
                let apos = self.dancers.iter().position(|x| x == a).unwrap_or_default();
                let bpos = self.dancers.iter().position(|x| x == b).unwrap_or_default();
                (self.dancers[apos], self.dancers[bpos]) = (self.dancers[bpos], self.dancers[apos]);
            },
        }
    }
}

impl ToString for Dancers {
    fn to_string(&self) -> String {
        self.dancers.iter().collect()
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = &s[1..];

        match s.chars().next() {
            Some('s') => rest.parse().map(|x| Move::Spin(x)).map_err(|_| ()),
            Some('x') => { 
                let (x,y) = rest.split_once('/').ok_or(())?;
                let x = x.parse().map_err(|_| ())?;
                let y = y.parse().map_err(|_| ())?;
                Ok(Move::Exchange(x, y))
            },
            Some('p') => {
                let (x,y) = rest.split_once('/').ok_or(())?;
                let c1 = x.chars().next().ok_or(())?;
                let c2 = y.chars().next().ok_or(())?;
                Ok(Move::Partner(c1, c2))
            },
            _ => Err(()),
        }
    }
}

fn parse_input(s: DayString) -> Dance {
    s.split(',').flat_map(|mv| mv.parse()).collect()
}

fn solve_part1(moves: &[Move]) -> Dancers {
    let mut dancers = Dancers::new();
    moves.iter().for_each(|x| dancers.dance_move(x));
    dancers
}

fn solve_part2(dancers: &mut Dancers, moves: &[Move]) {
    let amount = 1000000000;
    let mut ran = moves.len();

    let defstring = Dancers::new().to_string();
    while dancers.to_string() != defstring {
        moves.iter().for_each(|x| dancers.dance_move(x));
        ran += moves.len()
    }

    let amount = amount % ran;
    moves.iter().cycle().take(amount).for_each(|x| dancers.dance_move(x));
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let mut dancers = solve_part1(&parsed);
    let sol1 = dancers.to_string();
    solve_part2(&mut dancers, &parsed);
    let sol2 = dancers.to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(16))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = helpers::read_day(16);
        parse_input(input);
    }

    #[test]
    fn test_part1() {
        let mut dancers = Dancers::new();
        dancers.dance_move(&Move::Spin(1));
        assert_eq!(dancers.to_string(), "pabcdefghijklmno");

        let mut dancers = Dancers { dancers: vec!['a', 'b', 'c', 'd', 'e'] };
        let moves = parse_input("s1,x3/4,pe/b");
        moves.iter().for_each(|mov| dancers.dance_move(mov));
        assert_eq!(dancers.to_string(), "baedc");

        let mut dancers = Dancers { dancers: vec!['a', 'b', 'c', 'd', 'e'] };
        dancers.dance_move(&Move::Spin(1));
        assert_eq!(dancers.to_string(), "eabcd");

        let mut dancers = Dancers::new();
        dancers.dance_move(&Move::Spin(4));
        assert_eq!(dancers.to_string(), "mnopabcdefghijkl");
    }
}

