use aoc2022::{Day, Solution};

fn main() -> anyhow::Result<()> {
    aoc2022::run_solution::<Day06>()
}

#[test]
fn test_day06_solution() {
    aoc2022::test_solution::<Day06>()
}

struct Day06;
impl Solution for Day06 {
    const DAY: aoc2022::Day = match Day::number(6) {
        Ok(day) => day,
        _ => unreachable!(),
    };
    type Output1 = usize;
    type Output2 = usize;
    fn solve(input: &str) -> anyhow::Result<(Self::Output1, Self::Output2)> {
        let bytes = input.as_bytes();
        let mut p1 = 0;
        let mut p2 = 0;
        for (i, window) in bytes.windows(4).enumerate() {
            if is_all_unique_chars(window) {
                p1 = i + 4;
                break;
            }
        }
        for (i, window) in bytes.windows(14).enumerate() {
            if is_all_unique_chars(window) {
                p2 = i + 14;
                break;
            }
        }
        Ok((p1, p2))
    }
}

fn is_all_unique_chars(mut bytes: &[u8]) -> bool {
    while let Some((b, rest)) = bytes.split_first() {
        if rest.contains(b) {
            return false;
        }
        bytes = rest;
    }
    true
}
