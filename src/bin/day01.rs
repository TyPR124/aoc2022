use aoc2022::{Day, Solution};

fn main() -> anyhow::Result<()> {
    aoc2022::run_solution::<Day01>()
}

#[test]
fn test_day01_solution() {
    aoc2022::test_solution::<Day01>()
}

struct Day01;
impl Solution for Day01 {
    const DAY: aoc2022::Day = match Day::number(1) {
        Ok(day) => day,
        _ => unreachable!(),
    };
    type Output1 = usize;
    type Output2 = usize;
    fn solve(input: &str) -> anyhow::Result<(Self::Output1, Self::Output2)> {
        let mut first = 0;
        let mut second = 0;
        let mut third = 0;
        let mut sum = 0;
        for line in input.lines().chain(Some("")) {
            if line.is_empty() {
                if sum > first {
                    third = second;
                    second = first;
                    first = sum;
                } else if sum > second {
                    third = second;
                    second = sum;
                } else if sum > third {
                    third = sum;
                }
                sum = 0;
            } else {
                sum += line.parse::<usize>()?;
            }
        }

        Ok((first, first + second + third))
    }
}
