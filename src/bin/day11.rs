use std::{mem, str::FromStr};

use anyhow::bail;
use aoc2022::{Day, Solution};

fn main() -> anyhow::Result<()> {
    aoc2022::run_solution::<Day11>()
}

#[test]
fn test_day11_solution() {
    aoc2022::test_solution::<Day11>()
}

struct Day11;
impl Solution for Day11 {
    const DAY: aoc2022::Day = match Day::number(11) {
        Ok(day) => day,
        _ => unreachable!(),
    };
    type Output1 = usize;
    type Output2 = usize;
    fn solve(input: &str) -> anyhow::Result<(Self::Output1, Self::Output2)> {
        let mut monkeys = vec![];
        let lines = &mut input.lines().peekable();
        while lines.peek().is_some() {
            monkeys.push(Monkey::from_lines(
                lines.take_while(|line| !line.is_empty()),
            ));
        }
        let mut monkeys_part2 = monkeys.clone();
        for _ in 0..20 {
            monkey_around(&mut monkeys);
        }
        let mut p1_max1 = 0;
        let mut p1_max2 = 0;

        for monkey in &monkeys {
            if monkey.inspection_count > p1_max1 {
                p1_max2 = p1_max1;
                p1_max1 = monkey.inspection_count;
            } else if monkey.inspection_count > p1_max2 {
                p1_max2 = monkey.inspection_count;
            }
        }
        let divisor = monkeys_part2.iter().map(|m| m.test.divisible_by).product();
        for _ in 0..10_000 {
            monkey_around2(&mut monkeys_part2, divisor);
        }
        let mut p2_max1 = 0;
        let mut p2_max2 = 0;
        for monkey in &monkeys_part2 {
            if monkey.inspection_count > p2_max1 {
                p2_max2 = p2_max1;
                p2_max1 = monkey.inspection_count;
            } else if monkey.inspection_count > p2_max2 {
                p2_max2 = monkey.inspection_count;
            }
        }
        Ok((p1_max1 * p1_max2, p2_max1 * p2_max2))
    }
}
#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: Test,
    inspection_count: usize,
}
#[derive(Copy, Clone, Debug)]
enum Operation {
    Mul(Value, Value),
    Add(Value, Value),
}
#[derive(Copy, Clone, Debug)]
enum Value {
    Old,
    Const(usize),
}
#[derive(Copy, Clone, Debug)]
struct Test {
    divisible_by: usize,
    on_true: usize,
    on_false: usize,
}

impl Monkey {
    pub fn from_lines<'input>(mut lines: impl Iterator<Item = &'input str>) -> Self {
        let _ = lines.next().unwrap();
        let items = lines.next().unwrap()[18..]
            .split(", ")
            .map(|item| item.parse())
            .collect::<Result<_, _>>()
            .unwrap();
        let operation = lines.next().unwrap()[19..].parse().unwrap();
        let divisible_by = lines.next().unwrap()[21..].parse().unwrap();
        let on_true = lines.next().unwrap()[29..].parse().unwrap();
        let on_false = lines.next().unwrap()[30..].parse().unwrap();
        assert!(lines.next().is_none());
        Monkey {
            items,
            operation,
            test: Test {
                divisible_by,
                on_true,
                on_false,
            },
            inspection_count: 0,
        }
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, rest) = s.split_once(' ').unwrap();
        let (op, b) = rest.split_once(' ').unwrap();
        let a = a.parse()?;
        let b = b.parse()?;
        match op {
            "*" => Ok(Operation::Mul(a, b)),
            "+" => Ok(Operation::Add(a, b)),
            _ => bail!("invalid op"),
        }
    }
}

impl FromStr for Value {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Value::Old),
            _ => Ok(Value::Const(s.parse()?)),
        }
    }
}

fn monkey_around(monkeys: &mut [Monkey]) {
    for m in 0..monkeys.len() {
        monkeys[m].inspection_count += monkeys[m].items.len();
        let items = mem::take(&mut monkeys[m].items);
        for item in items {
            let item = monkeys[m].operation.calculate(item) / 3;
            let to = match 0 == item % monkeys[m].test.divisible_by {
                true => monkeys[m].test.on_true,
                false => monkeys[m].test.on_false,
            };
            monkeys[to].items.push(item)
        }
    }
}

fn monkey_around2(monkeys: &mut [Monkey], divisor: usize) {
    for m in 0..monkeys.len() {
        monkeys[m].inspection_count += monkeys[m].items.len();
        let items = mem::take(&mut monkeys[m].items);
        for item in items {
            let item = monkeys[m].operation.calculate(item) % divisor;
            let to = match 0 == item % monkeys[m].test.divisible_by {
                true => monkeys[m].test.on_true,
                false => monkeys[m].test.on_false,
            };
            monkeys[to].items.push(item)
        }
    }
}

impl Operation {
    pub fn calculate(&self, old: usize) -> usize {
        match self {
            Operation::Mul(a, b) => a.const_or_old(old) * b.const_or_old(old),
            Operation::Add(a, b) => a.const_or_old(old) + b.const_or_old(old),
        }
    }
}

impl Value {
    pub fn const_or_old(&self, old: usize) -> usize {
        match self {
            Value::Old => old,
            Value::Const(n) => *n,
        }
    }
}
