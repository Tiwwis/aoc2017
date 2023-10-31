use crate::helpers::{self, DayString, Solution};

use nom::{IResult, Parser};

use nom::character::complete::{char, none_of};
use nom::multi::{fold_many0, separated_list0};
use nom::sequence::delimited;

#[derive(Debug)]
enum Stream {
    Group(Vec<Stream>),
    Garbage(u16),
}

fn stream_parser(rem: &str) -> IResult<&str, Stream> {
    group_parser.or(garbage_parser).parse(rem)
}

fn in_garbage(rem: &str) -> IResult<&str, u16> {
    let (input, c) = none_of(">")(rem)?;
    if c == '!' {
        return Ok((&input[1..], 0));
    }
    Ok((input, 1))
}

fn garbage_parser(rem: &str) -> IResult<&str, Stream> {
    let (input, l) = delimited(
        char('<'),
        fold_many0(in_garbage, u16::default, |a, b| a + b),
        char('>'),
    )(rem)?;
    Ok((input, Stream::Garbage(l)))
}

fn group_parser(rem: &str) -> IResult<&str, Stream> {
    let (input, v) = delimited(
        char('{'),
        separated_list0(char(','), stream_parser),
        char('}'),
    )(rem)?;
    Ok((input, Stream::Group(v)))
}

fn parse_input(s: DayString) -> Stream {
    stream_parser(s).unwrap().1
}

fn solve_part1(input: &Stream) -> u16 {
    fn total_score(stream: &Stream, depth: u16) -> u16 {
        match stream {
            Stream::Garbage(_) => 0,
            Stream::Group(v) => depth + v.iter().map(|x| total_score(x, depth + 1)).sum::<u16>(),
        }
    }

    total_score(input, 1)
}

fn solve_part2(input: &Stream) -> u16 {
    fn total_garbage(stream: &Stream) -> u16 {
        match stream {
            Stream::Garbage(i) => *i,
            Stream::Group(v) => v.iter().map(total_garbage).sum::<u16>(),
        }
    }

    total_garbage(input)
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let sol1 = solve_part1(&parsed).to_string();
    let sol2 = solve_part2(&parsed).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(9))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input1 = parse_input("{{<a>},{<a>},{<a>},{<a>}}");
        assert_eq!(solve_part1(&input1), 9);
    }

    #[test]
    fn test_part2() {
        let input1 = parse_input("{{<a>},{<a>},{<a>},{<a>}}");
        assert_eq!(solve_part2(&input1), 4);
    }
}
