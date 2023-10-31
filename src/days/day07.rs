use crate::helpers;
use crate::helpers::{DayString, Solution};

use regex::Regex;
use std::collections::hash_map::{Entry, HashMap};

type Name = &'static str;

#[derive(Debug)]
struct Disk {
    name: Name,
    weight: u16,
    above: Vec<Disk>,
}

#[derive(Debug)]
struct Tower {
    root: Disk,
}

fn parse_input(s: DayString) -> Tower {
    fn initialise_disk(
        name: Name,
        un_disks: &mut HashMap<Name, (u16, Vec<Name>)>,
        fin_disks: &mut HashMap<Name, Disk>,
    ) -> Option<Disk> {
        if let Entry::Occupied(entry) = fin_disks.entry(name) {
            return Some(entry.remove());
        }

        if let Entry::Occupied(entry) = un_disks.entry(name) {
            let (weight, children) = entry.remove();
            let above = children
                .iter()
                .filter_map(|child| initialise_disk(child, un_disks, fin_disks))
                .collect();
            return Some(Disk {
                name,
                weight,
                above,
            });
        }

        None
    }

    let re = Regex::new(
        r"(?m)^(?P<name>\w+) \(:?(?P<weight>\d+)\)(:? -> (?P<children>(:?(:?\w+(, )?))*))?$",
    )
    .unwrap();

    let mut names = Vec::new();
    let mut uninitialized_disks = HashMap::new();

    for caps in re.captures_iter(s) {
        let name = caps.name("name").unwrap().as_str();
        let weight: u16 = caps["weight"].parse().unwrap();
        let children: Vec<Name> = caps
            .name("children")
            .map(|mat| mat.as_str().split(", ").collect())
            .unwrap_or_default();

        names.push(name);
        uninitialized_disks.insert(name, (weight, children));
    }

    let mut initialized_disks = HashMap::new();

    for name in names {
        if let Some(disk) = initialise_disk(name, &mut uninitialized_disks, &mut initialized_disks)
        {
            initialized_disks.insert(name, disk);
        };
    }

    if let Some(root) = initialized_disks.into_values().next() {
        return Tower { root };
    }

    panic!("Invalid input, no root found")
}

fn solve_part1(input: &Tower) -> &str {
    input.root.name
}

fn solve_part2(input: &Tower) -> usize {
    fn find_unbalanced(disk: &Disk) -> Result<usize, usize> {
        let results: Result<Vec<usize>, usize> = disk.above.iter().map(find_unbalanced).collect();
        let results: Vec<usize> = results?;

        let maj_weight: usize = results.get(1).copied().unwrap_or_default();

        for (i, &weight) in results.iter().enumerate() {
            if weight != maj_weight {
                let maj_weight = if i > 1 {
                    results[i - 1]
                } else {
                    results[i + 1]
                };
                return Err((disk.above[i].weight as usize + maj_weight) - weight);
            }
        }

        Ok(disk.above.len() * maj_weight + disk.weight as usize)
    }

    if let Err(n) = find_unbalanced(&input.root) {
        return n;
    }

    panic!("No unbalancedness found!")
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let sol1 = solve_part1(&parsed).to_string();
    let sol2 = solve_part2(&parsed).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(7))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let _input = parse_input(helpers::read_example("07"));
    }

    #[test]
    fn test_part1() {
        let input = parse_input(helpers::read_example("07"));
        assert_eq!(solve_part1(&input), "tknk");
    }

    #[test]
    fn test_part2() {
        let input = parse_input(helpers::read_example("07"));
        assert_eq!(solve_part2(&input), 60);
    }
}
