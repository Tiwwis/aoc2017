use std::ops::{Index, IndexMut};
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;

use std::time::Duration;

use crate::helpers;
use crate::helpers::{DayString, Solution};

const WAIT:Duration = Duration::from_millis(1);
const P:Reg = Reg { address:15 };

type Program = Vec<Command>;
#[derive(Debug)]
struct Memory([i128; 26]);

impl Index<Reg> for Memory {
    type Output = i128;

    fn index(&self, index: Reg) -> &Self::Output {
        &self.0[index.address]
    }
}

impl IndexMut<Reg> for Memory {
    fn index_mut(&mut self, index: Reg) -> &mut Self::Output {
        &mut self.0[index.address]
    }
}

impl Memory {
    fn get(&self, index: Reg) -> Option<&i128> {
        if index.address <= 25 {
            Some(&self[index])
        } else {
            None
        }
    }

    fn new(val: i128) -> Memory {
        Memory([val; 26])
    }
}

#[derive(Debug)]
struct CPU {
    program: Program,
    pointer: usize,
    memory: Memory,
    last_sound: i128,
    channel: Option<(mpsc::Sender<i128>, mpsc::Receiver<i128>)>,
    name: String,
}

#[derive(Debug, Clone, Copy)]
enum Value {
    CONST(i128),
    REG(Reg),
}

#[derive(Debug, Clone, Copy)]
struct Reg {
    address: usize,
}

#[derive(Debug, Clone, Copy)]
enum Command {
    SND(Value),
    SET(Reg, Value),
    ADD(Reg, Value),
    MUL(Reg, Value),
    MOD(Reg, Value),
    RCV(Reg),
    JGZ(Value, Value),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    DONE,
    RCV(i128),
    SENDING,
    RUNNING,
    WAITING,
    CANCELLED,
}

impl CPU {
    fn new(program: Program, name: String) -> CPU {
        let memory = Memory::new(0);
        CPU {
            program,
            pointer: 0,
            memory,
            last_sound: 0,
            channel: None,
            name,
        }
    }

    fn new_multi(
        program: Program,
        val: i128,
        chan: (mpsc::Sender<i128>, mpsc::Receiver<i128>),
        name: String,
    ) -> CPU {
        let mut cpu = Self::new(program, name);
        cpu.memory[P]=val;
        cpu.channel = Some(chan);
        cpu
    }

    fn compute_step(&mut self) -> Status {
        let program = &self.program;
        let pointer = self.pointer.clone();
        self.pointer += 1;

        match program.get(pointer).copied() {
            Some(Command::SND(val)) => {
                let loc = self.get_value(val);
                //println!("{}, {}", loc, &self.name);
                if let Some((tx, _)) = &self.channel {
                    tx.send(loc).unwrap();
                    return Status::SENDING;
                }
                self.last_sound = loc;
                Status::RUNNING
            }
            Some(Command::SET(reg, val)) => {
                let val = self.get_value(val);
                self.memory[reg] = val;
                Status::RUNNING
            }
            Some(Command::ADD(reg, val)) => {
                let val = self.get_value(val);
                self.memory[reg] += val;
                Status::RUNNING
            }

            Some(Command::MUL(reg, val)) => {
                let val = self.get_value(val);
                self.memory[reg] *= val;
                Status::RUNNING
            }

            Some(Command::MOD(reg, val)) => {
                let val = self.get_value(val);
                self.memory[reg] %= val;
                Status::RUNNING
            }
            Some(Command::RCV(reg)) => {
                if let Some((_, rx)) = &self.channel {
                    match rx.recv_timeout(WAIT) {
                        Ok(v) => {
                            self.memory[reg] = v;
                            return Status::WAITING;
                        }
                        Err(_) => return Status::CANCELLED,
                    }
                }
                let val = self.memory[reg];
                if val != 0 {
                    Status::RCV(self.last_sound)
                } else {
                    Status::RUNNING
                }
            }

            Some(Command::JGZ(cond, val)) => {
                let cond = self.get_value(cond);
                let val = self.get_value(val);
                if cond > 0 {
                    self.pointer = ((self.pointer as i128) + val - 1) as usize;
                }
                Status::RUNNING
            }
            _ => Status::DONE,
        }
    }

    fn get_value(&self, val: Value) -> i128 {
        match val {
            Value::CONST(x) => x,
            Value::REG(rg) => self.memory.get(rg).copied().unwrap_or_default(),
        }
    }
}

impl Iterator for CPU {
    type Item = Status;

    fn next(&mut self) -> Option<Self::Item> {
        match self.compute_step() {
            Status::DONE => None,
            x @ _ => Some(x),
        }
    }
}

impl FromStr for Reg {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let c = chars.next().ok_or(())?;

        if let Some(_) = chars.next() {
            return Err(());
        }

        if !c.is_ascii_lowercase() {
            return Err(());
        }
        Ok(Reg {
            address: (c as usize) - ('a' as usize),
        })
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(x) = s.parse() {
            return Ok(Value::CONST(x));
        }

        Ok(Self::REG(s.parse()?))
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();

        let first = words.next();
        let second = words.next();
        let third = words.next();

        match (first, second, third) {
            (Some("snd"), Some(s), None) => Ok(Self::SND(s.parse()?)),
            (Some("set"), Some(s1), Some(s2)) => Ok(Self::SET(s1.parse()?, s2.parse()?)),
            (Some("add"), Some(s1), Some(s2)) => Ok(Self::ADD(s1.parse()?, s2.parse()?)),
            (Some("mul"), Some(s1), Some(s2)) => Ok(Self::MUL(s1.parse()?, s2.parse()?)),
            (Some("mod"), Some(s1), Some(s2)) => Ok(Self::MOD(s1.parse()?, s2.parse()?)),
            (Some("rcv"), Some(s), None) => Ok(Self::RCV(s.parse()?)),
            (Some("jgz"), Some(s1), Some(s2)) => Ok(Self::JGZ(s1.parse()?, s2.parse()?)),
            _ => Err(()),
        }
    }
}

fn parse_input(s: DayString) -> Program {
    s.lines().flat_map(|s| s.parse()).collect()
}

fn solve_part1(input: &Program) -> i128 {
    let mut cpu = CPU::new(input.clone(), "".to_string());
    let result = cpu.find(|x| if let Status::RCV(_) = x { true } else { false });
    if let Status::RCV(x) = result.expect("should receive once!") {
        return x;
    }
    panic!("should be received!")
}

fn solve_part2(input: &Program) -> usize {
    let (t0, r1) = mpsc::channel();
    let (t1, r0) = mpsc::channel();

    let input0 = input.clone();
    let input1 = input.clone();

    let thread0 = thread::spawn(move || {
        let cpu0 = CPU::new_multi(input0, 0, (t0, r0), "cpu0".to_string());
        println!("HELLO FROM THREAD 0");
        let mut count = 0;
        for status in cpu0 {
            match status {
                Status::SENDING => { count+= 1; },
                Status::CANCELLED => break,
                _ => { () },
            }
        }
        count
    });

    let thread1 = thread::spawn(move || {
        let cpu1 = CPU::new_multi(input1, 1, (t1, r1), "cpu1".to_string());
        let mut count = 0;
        println!("HELLO FROM THREAD 1");
        for status in cpu1 {
            match status {
                Status::SENDING => { count+= 1; },
                Status::CANCELLED => break,
                _ => { () },
            }
        };
        count
    });

    let _count0 = thread0.join().expect("Calculation should finish without issue!");
    let count1 = thread1.join().expect("Calculation in Thread 1 encountered trouble!");
    count1
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let sol1 = solve_part1(&parsed).to_string();
    let sol2 = solve_part2(&parsed).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(18))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(helpers::read_example("18"));
        assert_eq!(solve_part1(&input), 4);
    }


    #[test]
    fn test_part2() {
        let input = parse_input(helpers::read_example("18_2"));
        assert_eq!(solve_part2(&input), 3);
    }
}
