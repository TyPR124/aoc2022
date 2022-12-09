use std::{cmp::Ordering, collections::HashSet};

use aoc2022::{Day, Solution};

fn main() -> anyhow::Result<()> {
    aoc2022::run_solution::<Day09>()
}

#[test]
fn test_day09_solution() {
    aoc2022::test_solution::<Day09>()
}

struct Day09;
impl Solution for Day09 {
    const DAY: aoc2022::Day = match Day::number(9) {
        Ok(day) => day,
        _ => unreachable!(),
    };
    type Output1 = usize;
    type Output2 = usize;
    fn solve(input: &str) -> anyhow::Result<(Self::Output1, Self::Output2)> {
        let mut head = (0, 0);
        let mut tails = [(0, 0); 9];
        let mut tail0_visited = HashSet::new();
        let mut tail8_visited = HashSet::new();
        for line in input.lines() {
            let (dir, count) = line.split_once(' ').unwrap();
            let count: usize = count.parse().unwrap();
            for _ in 0..count {
                match dir {
                    "U" => step(&mut head, &mut tails, 0, -1),
                    "D" => step(&mut head, &mut tails, 0, 1),
                    "L" => step(&mut head, &mut tails, -1, 0),
                    "R" => step(&mut head, &mut tails, 1, 0),
                    _ => unreachable!("invalid input data"),
                }
                tail0_visited.insert(tails[0]);
                tail8_visited.insert(tails[8]);
            }
        }
        Ok((tail0_visited.len(), tail8_visited.len()))
    }
}

fn step(head: &mut (isize, isize), tails: &mut [(isize, isize)], dx: isize, dy: isize) {
    head.0 += dx;
    head.1 += dy;

    step_tail(*head, &mut tails[0]);
    step_tail(tails[0], &mut tails[1]);
    step_tail(tails[1], &mut tails[2]);
    step_tail(tails[2], &mut tails[3]);
    step_tail(tails[3], &mut tails[4]);
    step_tail(tails[4], &mut tails[5]);
    step_tail(tails[5], &mut tails[6]);
    step_tail(tails[6], &mut tails[7]);
    step_tail(tails[7], &mut tails[8]);
}

fn step_tail(head: (isize, isize), tail: &mut (isize, isize)) {
    if !is_touching(head, *tail) {
        match tail.0.cmp(&head.0) {
            Ordering::Less => tail.0 += 1,
            Ordering::Equal => (),
            Ordering::Greater => tail.0 -= 1,
        }
        match tail.1.cmp(&head.1) {
            Ordering::Less => tail.1 += 1,
            Ordering::Equal => (),
            Ordering::Greater => tail.1 -= 1,
        }
    }
}

fn is_touching(head: (isize, isize), tail: (isize, isize)) -> bool {
    head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1
}
