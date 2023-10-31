use crate::helpers;
use crate::helpers::{DayString, Solution};
use regex::Regex;

type Node = usize;
type Graph = Vec<Vec<Node>>;

fn parse_input(s: DayString) -> Graph {
    let re = Regex::new(r"(?m)^(\d+) <-> ((?:\d+(?:, )?)*)$").unwrap();
    let mut all = Vec::new();
    for cap in re.captures_iter(s) {
        let neighbs = cap
            .get(2)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        all.push(neighbs);
    }
    all
}

fn dfs(g: &Graph, s: Node, found: &mut [bool]) {
    found[s] = true;
    for nb in &g[s] {
        if !found[*nb] {
            dfs(g, *nb, found)
        }
    }
}
fn solve_part1(graph: &Graph) -> usize {
    let n = graph.len();
    let mut found = vec![false; n];
    dfs(graph, 0, &mut found);
    found.into_iter().filter(|&b| b).count()
}

fn solve_part2(graph: &Graph) -> usize {
    let n = graph.len();
    let mut found = vec![false; n];
    let mut counter = 0;
    for i in 0..n {
        if !found[i] {
            dfs(graph, i, &mut found);
            counter += 1
        }
    }
    counter
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let sol1 = solve_part1(&parsed).to_string();
    let sol2 = solve_part2(&parsed).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(12))
}
