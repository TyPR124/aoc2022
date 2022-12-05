use aoc2022::{Day, Solution};

fn main() -> anyhow::Result<()> {
    aoc2022::run_solution::<Day03>()
}

#[test]
fn test_day03_solution() {
    aoc2022::test_solution::<Day03>()
}

struct Day03;
impl Solution for Day03 {
    const DAY: aoc2022::Day = match Day::number(3) {
        Ok(day) => day,
        _ => unreachable!(),
    };
    type Output1 = usize;
    type Output2 = usize;
    fn solve(input: &str) -> anyhow::Result<(Self::Output1, Self::Output2)> {
        let mut part1_sum = 0;
        let mut part2_sum = 0;
        let mut full_sets = vec![];
        for line in input.lines() {
            let bytes = line.as_bytes();
            let (half1, half2) = bytes.split_at(bytes.len() / 2);
            let (mut pset1, mut pset2) = ([false; 52], [false; 52]);
            fill_priority_set(half1, &mut pset1);
            fill_priority_set(half2, &mut pset2);
            part1_sum += common_priority(&pset1, &pset2).expect("missing common priority") as usize;
            full_sets.push(combine_sets(&pset1, &pset2));
        }
        for group in full_sets.chunks(3) {
            part2_sum += common_priority3(&group[0], &group[1], &group[2])
                .expect("missing common priority in group") as usize;
        }
        Ok((part1_sum, part2_sum))
    }
}

fn priority(c: u8) -> u8 {
    if c >= b'a' {
        c - b'a' + 1
    } else {
        c - b'A' + 27
    }
}

fn fill_priority_set(bytes: &[u8], pset: &mut [bool; 52]) {
    for &byte in bytes {
        let priority = priority(byte);
        pset[priority as usize - 1] = true;
    }
}

fn common_priority(pset1: &[bool; 52], pset2: &[bool; 52]) -> Option<u8> {
    pset1
        .iter()
        .zip(pset2.iter())
        .enumerate()
        .find(|&(_, (&s1, &s2))| s1 && s2)
        .map(|(p, _)| p as u8 + 1)
}

fn common_priority3(pset1: &[bool; 52], pset2: &[bool; 52], pset3: &[bool; 52]) -> Option<u8> {
    pset1
        .iter()
        .copied()
        .zip(pset2.iter().copied())
        .zip(pset3.iter().copied())
        .enumerate()
        .find(|&(_, ((p1, p2), p3))| p1 && p2 && p3)
        .map(|(p, _)| p as u8 + 1)
}

fn combine_sets(pset1: &[bool; 52], pset2: &[bool; 52]) -> [bool; 52] {
    std::array::from_fn(|i| pset1[i] || pset2[i])
}
