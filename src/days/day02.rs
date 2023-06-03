use crate::helpers;
use crate::helpers::{DayString, Solution};

type Spreadsheet = Vec<Vec<u32>>;
fn parse_input(s: DayString) -> Spreadsheet {

    fn parse_line(line:&str) -> Vec<u32> {
        line.split(char::is_whitespace).map(|x| x.parse().unwrap()).collect()
    }

    s.lines().map(parse_line).collect()
}

fn solve_part1(sheet: &Spreadsheet) -> u32 {

    fn row_diff(row: &Vec<u32>) -> Option<u32> {
        let max = row.iter().max()?;
        let min = row.iter().min()?;
        Some(max - min)
    }

    sheet.iter().map(row_diff).flatten().sum::<u32>()
}

fn solve_part2(sheet: &Spreadsheet) -> u32 {

    fn row_diff(row: &Vec<u32>) -> Option<u32> {
        let n = row.len();
        for i in 0..n {
            for j in i+1..n {
                let n = row[i];
                let m = row[j];
                let (n, m) = (u32::max(n,m), u32::min(n,m));
                if (n % m) == 0 { return Some(n/m) }
            }
        }
        None
    }

    sheet.iter().map(row_diff).flatten().sum::<u32>()
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let sol1 = solve_part1(&parsed).to_string();
    let sol2 = solve_part2(&parsed).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let ex1:Spreadsheet = parse_input(helpers::read_example("02"));
        assert_eq!(solve_part1(&ex1), 18);
    }

    #[test]
    fn test_part2() {
        let ex1:Spreadsheet = parse_input(helpers::read_example("02"));
        assert_eq!(solve_part2(&ex1), 9);
    }
}
