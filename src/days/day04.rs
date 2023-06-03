use crate::helpers;
use crate::helpers::{DayString, Solution};

type PassPhrase<'a> = Vec<&'a str>;
type Input = Vec<PassPhrase<'static>>;

fn parse_input(s: DayString) -> Input {
    s.lines().map(|l| l.trim().split(' ').collect()).collect()
}

fn no_duplicates<T: AsRef<str>>(phrase:&Vec<T>) -> bool {
    let mut new_phrase:Vec<&str> = phrase.iter().map(T::as_ref).collect();
    new_phrase.sort_unstable();
    new_phrase.dedup();
    new_phrase.len() == phrase.len()
}

fn anagram_free(phrase:&PassPhrase) -> bool {

    fn word_order(word:&str) -> String {
        let mut new_word:Vec<char> = word.chars().collect();
        new_word.sort();
        new_word.into_iter().collect()
    }

    let new_words:Vec<String> = phrase.iter().copied().map(word_order).collect();
    no_duplicates(&new_words)
}

fn solve_part1(input: &Input) -> usize {
    input.iter().filter(|&p| no_duplicates(p)).count()
}

fn solve_part2(input: &Input) -> usize {
    input.iter().filter(|&p| anagram_free(p)).count()
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let sol1 = solve_part1(&parsed).to_string();
    let sol2 = solve_part2(&parsed).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(4))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(helpers::read_example("04-1"));
        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(helpers::read_example("04-2"));
        assert_eq!(solve_part2(&input), 3);
    }
}

