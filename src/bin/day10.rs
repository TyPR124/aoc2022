use aoc2022::{Day, Solution};

fn main() -> anyhow::Result<()> {
    aoc2022::run_solution::<Day10>()
}

#[test]
fn test_day10_solution() {
    aoc2022::test_solution::<Day10>()
}

struct Day10;
impl Solution for Day10 {
    const DAY: aoc2022::Day = match Day::number(10) {
        Ok(day) => day,
        _ => unreachable!(),
    };
    type Output1 = isize;
    type Output2 = usize;
    fn solve(input: &str) -> anyhow::Result<(Self::Output1, Self::Output2)> {
        let mut cycles = 0;
        let mut x = 1;
        let mut part1 = 0;
        let mut pixels = [[' '; 40]; 6];
        for line in input.lines() {
            let start_cycle = cycles;
            let start_x = x;
            if line == "noop" {
                cycles += 1;
            } else if let Some(("addx", n)) = line.split_once(' ') {
                let n: isize = n.parse().unwrap();
                cycles += 2;
                x += n;
            } else {
                unreachable!("invalid input data")
            }
            if (cycles % 20) < (start_cycle % 20) {
                let c = (cycles / 20) * 20;
                if (cycles / 20) % 2 == 1 {
                    part1 += c * start_x
                }
            }
            for c in (start_cycle)..cycles {
                let y = c / 40;
                let x = c % 40;
                if start_x.abs_diff(x) <= 1 {
                    pixels[y as usize][x as usize] = '█'
                }
            }
        }
        for row in &pixels {
            for &p in row {
                print!("{}", p)
            }
            println!();
        }
        Ok((part1, 0))
    }
}
