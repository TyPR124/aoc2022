use std::mem;

use aoc2022::{Day, Solution};

fn main() -> anyhow::Result<()> {
    aoc2022::run_solution::<Day12>()
}

#[test]
fn test_day12_solution() {
    aoc2022::test_solution::<Day12>()
}

struct Day12;
impl Solution for Day12 {
    const DAY: aoc2022::Day = match Day::number(12) {
        Ok(day) => day,
        _ => unreachable!(),
    };
    type Output1 = usize;
    type Output2 = usize;
    fn solve(input: &str) -> anyhow::Result<(Self::Output1, Self::Output2)> {
        let mut grid: Vec<Vec<(u8, bool)>> = input
            .lines()
            .map(|line| line.bytes().map(|b| (b, false)).collect())
            .collect();
        let mut start = None;
        let mut end = None;
        let width = grid[0].len();
        let height = grid.len();

        'find_start_end: for y in 0..height {
            for x in 0..width {
                if grid[y][x].0 == b'S' {
                    grid[y][x].0 = b'a';
                    start = Some((x, y));
                    if end.is_some() {
                        break 'find_start_end;
                    }
                } else if grid[y][x].0 == b'E' {
                    grid[y][x].0 = b'z';
                    end = Some((x, y));
                    if start.is_some() {
                        break 'find_start_end;
                    }
                }
            }
        }
        let mut grid_part2 = grid.clone();
        let start = start.unwrap();
        let end = end.unwrap();
        grid[start.1][start.0].1 = true;

        let mut to_visit = vec![start];
        let mut cost = 0;
        let part1 = 'part1: loop {
            cost += 1;
            let next_set = mem::take(&mut to_visit);
            for (cx, cy) in next_set {
                for (x, y) in valid_moves(&mut grid, cx, cy) {
                    if end == (x, y) {
                        break 'part1 cost;
                    } else {
                        to_visit.push((x, y))
                    }
                }
            }
        };

        let mut to_visit = vec![end];
        grid_part2[end.1][end.0].1 = true;
        let mut cost = 0;
        let part2 = 'part2: loop {
            cost += 1;
            let next_set = mem::take(&mut to_visit);
            for (cx, cy) in next_set {
                for (x, y, h) in valid_moves_reversed(&mut grid_part2, cx, cy) {
                    if h == b'a' {
                        break 'part2 cost;
                    } else {
                        to_visit.push((x, y))
                    }
                }
            }
        };

        Ok((part1, part2))
    }
}

fn valid_moves<'grid>(
    grid: &'grid mut [Vec<(u8, bool)>],
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize)> + 'grid {
    let height = grid[y][x].0;

    [
        (x.wrapping_sub(1), y),
        (x.wrapping_add(1), y),
        (x, y.wrapping_sub(1)),
        (x, y.wrapping_add(1)),
    ]
    .into_iter()
    .filter_map(move |(x, y)| {
        let (h, v) = grid.get_mut(y).and_then(|row| row.get_mut(x))?;
        (!*v && (*h < height || h.abs_diff(height) <= 1)).then(|| {
            *v = true;
            (x, y)
        })
    })
}

fn valid_moves_reversed<'grid>(
    grid: &'grid mut [Vec<(u8, bool)>],
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize, u8)> + 'grid {
    let height = grid[y][x].0;

    [
        (x.wrapping_sub(1), y),
        (x.wrapping_add(1), y),
        (x, y.wrapping_sub(1)),
        (x, y.wrapping_add(1)),
    ]
    .into_iter()
    .filter_map(move |(x, y)| {
        let (h, v) = grid.get_mut(y).and_then(|row| row.get_mut(x))?;
        (!*v && (*h > height || h.abs_diff(height) <= 1)).then(|| {
            *v = true;
            (x, y, *h)
        })
    })
}
