use std::cmp::Ordering;

use aoc2022::{Day, Solution};

fn main() -> anyhow::Result<()> {
    aoc2022::run_solution::<Day05>()
}

#[test]
fn test_day05_solution() {
    aoc2022::test_solution::<Day05>()
}

struct Day05;
impl Solution for Day05 {
    const DAY: aoc2022::Day = match Day::number(5) {
        Ok(day) => day,
        _ => unreachable!(),
    };
    type Output1 = String;
    type Output2 = String;
    fn solve(input: &str) -> anyhow::Result<(Self::Output1, Self::Output2)> {
        let mut lines = input.lines();
        // Build the stacks
        let mut stacks = vec![];
        for line in &mut lines {
            let bytes = line.as_bytes();
            let mut bytes = bytes.iter();
            let &first = bytes.nth(1).unwrap();
            let mut stack = 0;
            if first == b'1' {
                break;
            }
            push_to_stacks(first, &mut stacks, stack);
            while let Some(&next) = bytes.nth(3) {
                stack += 1;
                push_to_stacks(next, &mut stacks, stack)
            }
        }
        stacks.iter_mut().for_each(|v| v.reverse());
        stacks.iter_mut().for_each(|v| {
            while v.last() == Some(&b' ') {
                v.pop();
            }
        });
        let (mut stacks_part1, mut stacks_part2) = (stacks.clone(), stacks);
        lines.next().unwrap();
        // Now process all the moves
        for line in &mut lines {
            let StackMove { count, from, to } = move_parser::stack_move(line).unwrap();
            let mut part2_moving = vec![];
            (0..count).for_each(|_| {
                let item = stacks_part1[from].pop().unwrap();
                stacks_part1[to].push(item);

                let item = stacks_part2[from].pop().unwrap();
                part2_moving.insert(0, item);
            });
            stacks_part2[to].extend_from_slice(&part2_moving);
            part2_moving.clear();
        }
        let mut part1 = String::new();
        let mut part2 = String::new();
        for s in &stacks_part1 {
            part1.push(*s.last().unwrap() as char)
        }
        for s in &stacks_part2 {
            part2.push(*s.last().unwrap() as char)
        }
        Ok((part1, part2))
    }
}

fn push_to_stacks(byte: u8, stacks: &mut Vec<Vec<u8>>, stack: usize) {
    match stack.cmp(&stacks.len()) {
        Ordering::Less => stacks[stack].push(byte),
        Ordering::Equal => stacks.push(vec![byte]),
        Ordering::Greater => panic!("stack out of range"),
    }
}

pub struct StackMove {
    count: usize,
    from: usize,
    to: usize,
}

peg::parser! {
  grammar move_parser() for str {
    pub rule stack_move() -> StackMove
      = "move " count:number() " from " from:number() " to " to:number() { StackMove { count, from: from - 1, to: to - 1 }}

    rule number() -> usize
      = n:$(['0'..='9']+) { n.parse().unwrap() }
  }
}
