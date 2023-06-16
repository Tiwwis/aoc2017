use crate::helpers;
use crate::helpers::{DayString, Solution};

use std::str::FromStr;
use std::collections::hash_map::HashMap;

use regex::Regex;

type Reg = &'static str;
type Program = Vec<Instruction>;
type MemTable = HashMap<Reg, i32>;

#[derive(Clone, Copy)]
enum Operation { Inc, Dec }

#[derive(Clone, Copy)]
enum Cond { Eq, Leq, Lt, Gt, Geq, Neq }

impl FromStr for Operation {
    type Err = ();

    fn from_str(s:&str) -> Result<Self, Self::Err> {
        match s {
            "inc" => Ok(Operation::Inc),
            "dec" => Ok(Operation::Dec),
            _ => Err(())
        }
    }
}


impl FromStr for Cond {
    type Err = ();

    fn from_str(s:&str) -> Result<Self, Self::Err> {
        match s {
            "==" => Ok(Cond::Eq),
            "<=" => Ok(Cond::Leq),
            "<" => Ok(Cond::Lt),
            ">=" => Ok(Cond::Geq),
            ">" => Ok(Cond::Gt),
            "!=" => Ok(Cond::Neq),
            _ => Err(())
        }
    }
}

struct Instruction {
    target : Reg,
    op : Operation,
    by : i32,
    cond : Cond,
    cmp_targ : Reg,
    cmp_val : i32,
}

struct Memory {
    table: MemTable
}

impl Memory {
    fn new() -> Memory {
        Memory { table:MemTable::new() }
    }

    fn run_instruction(&mut self, cmd:&Instruction) -> Option<i32>{
        let cmp_targ = *self.table.entry(cmd.cmp_targ).or_default();
        let cond = cmd.cond;
        let cmp_val = cmd.cmp_val;

        let run_op = match cond {
            Cond::Eq => cmp_targ == cmp_val,
            Cond::Leq => cmp_targ <= cmp_val,
            Cond::Lt => cmp_targ < cmp_val,
            Cond::Gt => cmp_targ > cmp_val,
            Cond::Geq => cmp_targ >= cmp_val,
            Cond::Neq => cmp_targ != cmp_val,
        };

        if run_op {
            let op_targ = self.table.entry(cmd.target).or_default();
            match cmd.op {
                Operation::Inc => *op_targ += cmd.by,
                Operation::Dec => *op_targ -= cmd.by,
            }
            Some(*op_targ)
        } else {
            None
        }
    }
}

fn parse_input(s: DayString) -> Program {

    let mut v:Program = Vec::new();

    let re = Regex::new(r"(?m)^(?P<tar>\w+) (?P<op>inc|dec) (?P<by>(?:-)?\d+) if (?P<cpt>\w+) (?P<cmp>==|<|<=|>|>=|!=) (?P<cpv>(?:-)?\d+)$").unwrap();
    for cap in re.captures_iter(s) {
        let target = cap.name("tar").unwrap().as_str();
        let op = cap.name("op").unwrap().as_str().parse().unwrap();
        let by = cap.name("by").unwrap().as_str().parse().unwrap();
        let cmp_targ = cap.name("cpt").unwrap().as_str();
        let cond = cap.name("cmp").unwrap().as_str().parse().unwrap();
        let cmp_val = cap.name("cpv").unwrap().as_str().parse().unwrap();

        v.push(Instruction { target, op, by, cmp_targ, cond, cmp_val });
    }

    v
}

fn solve_day(input: &Program) -> (i32, i32) {
    let mut mem = Memory::new();
    let part2 = input.iter().filter_map(|instr| mem.run_instruction(instr)).max().unwrap_or_default();
    let part1 = mem.table.values().max().copied().unwrap_or_default();
    (part1, part2)
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let (sol1, sol2) = solve_day(&parsed);
    [sol1.to_string(), sol2.to_string()]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(8))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        let input = parse_input(helpers::read_example("08"));
        assert_eq!(solve_day(&input), (1,10));
    }
}

