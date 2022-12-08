use std::collections::HashSet;

use aoc2022::{Day, Solution};

fn main() -> anyhow::Result<()> {
    aoc2022::run_solution::<Day08>()
}

#[test]
fn test_day08_solution() {
    aoc2022::test_solution::<Day08>()
}

struct Day08;
impl Solution for Day08 {
    const DAY: aoc2022::Day = match Day::number(8) {
        Ok(day) => day,
        _ => unreachable!(),
    };
    type Output1 = usize;
    type Output2 = usize;
    fn solve(input: &str) -> anyhow::Result<(Self::Output1, Self::Output2)> {
        let mut seen = HashSet::new();
        let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
        let width = grid[0].len();
        let height = grid.len();
        for x in 0..width {
            visible_trees(&grid, x, 0, 0, 1, &mut seen);
            visible_trees(&grid, x, height - 1, 0, -1, &mut seen);
        }
        for y in 0..height {
            visible_trees(&grid, 0, y, 1, 0, &mut seen);
            visible_trees(&grid, width - 1, y, -1, 0, &mut seen);
        }
        let mut max_score = 0;
        for x in 0..width {
            for y in 0..height {
                let score = scenic_score(&grid, x, y);
                if score > max_score {
                    max_score = score;
                }
            }
        }
        Ok((seen.len(), max_score))
    }
}

fn visible_trees(
    grid: &[&[u8]],
    startx: usize,
    starty: usize,
    dx: isize,
    dy: isize,
    seen: &mut HashSet<(usize, usize)>,
) {
    let mut x = startx as isize;
    let mut y = starty as isize;
    let mut max_height = -1;

    while let Some(&tree) = grid.get(y as usize).and_then(|row| row.get(x as usize)) {
        if tree as isize > max_height {
            seen.insert((x as usize, y as usize));
            max_height = tree as isize;
        }
        x += dx;
        y += dy;
    }
}

fn scenic_score(grid: &[&[u8]], x: usize, y: usize) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let max_tree_height = grid[y][x];
    let mut blocked = false;

    let left = (0..x)
        .rev()
        .take_while(|&x| {
            if grid[y][x] < max_tree_height {
                true
            } else {
                blocked = true;
                false
            }
        })
        .count();
    let left = left + 1 * blocked as usize;
    blocked = false;
    let right = ((x + 1)..width)
        .take_while(|&x| {
            if grid[y][x] < max_tree_height {
                true
            } else {
                blocked = true;
                false
            }
        })
        .count();
    let right = right + 1 * blocked as usize;
    blocked = false;
    let up = (0..y)
        .rev()
        .take_while(|&y| {
            if grid[y][x] < max_tree_height {
                true
            } else {
                blocked = true;
                false
            }
        })
        .count();
    let up = up + 1 * blocked as usize;
    blocked = false;
    let down = ((y + 1)..height)
        .take_while(|&y| {
            if grid[y][x] < max_tree_height {
                true
            } else {
                blocked = true;
                false
            }
        })
        .count();
    let down = down + 1 * blocked as usize;

    left * right * up * down
}
