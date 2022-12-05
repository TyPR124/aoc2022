use std::ops::RangeInclusive;

use aoc2022::{Day, Solution};

fn main() -> anyhow::Result<()> {
    aoc2022::run_solution::<Day04>()
}

#[test]
fn test_day04_solution() {
    aoc2022::test_solution::<Day04>()
}

struct Day04;
impl Solution for Day04 {
    const DAY: aoc2022::Day = match Day::number(4) {
        Ok(day) => day,
        _ => unreachable!(),
    };
    type Output1 = usize;
    type Output2 = usize;
    fn solve(input: &str) -> anyhow::Result<(Self::Output1, Self::Output2)> {
        // 1-4,2-3
        let mut part1_sum = 0;
        let mut part2_sum = 0;
        for line in input.lines() {
            let mut parts = line.split(|c| matches!(c, '-' | ','));
            let r1 =
                parts.next().unwrap().parse().unwrap()..=parts.next().unwrap().parse().unwrap();
            let r2 =
                parts.next().unwrap().parse().unwrap()..=parts.next().unwrap().parse().unwrap();
            if is_range_fully_contained(&r1, &r2) {
                part1_sum += 1;
                part2_sum += 1;
            } else if is_range_partially_contained(&r1, &r2) {
                part2_sum += 1;
            }
        }
        Ok((part1_sum, part2_sum))
    }
}

fn is_range_fully_contained(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    (a.contains(b.start()) && a.contains(b.end())) || (b.contains(a.start()) && b.contains(a.end()))
}

fn is_range_partially_contained(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.contains(&b.start()) || a.contains(b.end()) || b.contains(a.start()) || b.contains(a.end())
}
