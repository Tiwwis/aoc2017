use crate::helpers;
use crate::helpers::{DayString, Solution};

type Captcha = Vec<u32>;

fn parse_input(s: DayString) -> Captcha {
    s.chars().map(|x| x.to_digit(10).unwrap()).collect()
}

fn solve_part1(captcha: &Captcha) -> u32 {
    let mut sum: u32 = 0;
    let n = captcha.len();
    for i in 0..n {
        let this = captcha[i];
        let next = captcha[(i + 1) % n];
        sum += ((this == next) as u32) * this
    }
    sum
}

fn solve_part2(captcha: &Captcha) -> u32 {
    let n = captcha.len();
    (0..n)
        .map(|i| {
            let this = captcha[i];
            let next = captcha[(i + n / 2) % n];
            ((this == next) as u32) * this
        })
        .sum::<u32>()
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let sol1 = solve_part1(&parsed).to_string();
    let sol2 = solve_part2(&parsed).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_input("1122"), [1, 1, 2, 2]);
    }

    #[test]
    fn test_part1() {
        let ex1 = parse_input("1122");
        let ex2 = parse_input("1111");
        let ex3 = parse_input("1234");
        let ex4 = parse_input("91212129");

        assert_eq!(solve_part1(&ex1), 3);
        assert_eq!(solve_part1(&ex2), 4);
        assert_eq!(solve_part1(&ex3), 0);
        assert_eq!(solve_part1(&ex4), 9);
    }

    #[test]
    fn test_part2() {
        let ex1 = parse_input("1212");
        let ex2 = parse_input("1221");
        let ex3 = parse_input("123425");
        let ex4 = parse_input("123123");
        let ex5 = parse_input("12131415");

        assert_eq!(solve_part2(&ex1), 6);
        assert_eq!(solve_part2(&ex2), 0);
        assert_eq!(solve_part2(&ex3), 4);
        assert_eq!(solve_part2(&ex4), 12);
        assert_eq!(solve_part2(&ex5), 4);
    }
}
