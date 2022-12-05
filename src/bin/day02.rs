use aoc2022::{Day, Solution};

use Choice::*;
use Outcome::*;

fn main() -> anyhow::Result<()> {
    aoc2022::run_solution::<Day02>()
}

#[test]
fn test_day02_solution() {
    aoc2022::test_solution::<Day02>()
}

struct Day02;
impl Solution for Day02 {
    const DAY: aoc2022::Day = match Day::number(2) {
        Ok(day) => day,
        _ => unreachable!(),
    };
    type Output1 = usize;
    type Output2 = usize;
    fn solve(input: &str) -> anyhow::Result<(Self::Output1, Self::Output2)> {
        let mut part1_score = 0;
        let mut part2_score = 0;
        for line in input.lines() {
            // Part 1 - Get my choice from X/Y/Z characters
            let bytes = line.as_bytes();
            let me = Choice::from_char(bytes[2])?;
            let opponent = Choice::from_char(bytes[0])?;
            part1_score += score(me, opponent);

            // Part 2 - Get the desired outcome from X/Y/Z characters
            let desired_outcome = Outcome::from_char(bytes[2])?;
            let me = Choice::from_outcome(opponent, desired_outcome);
            part2_score += score(me, opponent);
        }
        Ok((part1_score, part2_score))
    }
}
fn score(me: Choice, opponent: Choice) -> usize {
    me as usize + (3 * me.against(opponent) as usize)
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Choice {
    Rock = 1,
    Paper,
    Scissors,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Outcome {
    Lose = 0,
    Draw,
    Win,
}

impl Choice {
    pub fn against(self, other: Self) -> Outcome {
        match (self, other) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Lose,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
        }
    }
    pub fn from_char(c: u8) -> Result<Self, InvalidChoiceCharacter> {
        match c {
            b'A' | b'X' => Ok(Rock),
            b'B' | b'Y' => Ok(Paper),
            b'C' | b'Z' => Ok(Scissors),
            _ => Err(InvalidChoiceCharacter(c)),
        }
    }
    pub fn from_outcome(opponent: Choice, outcome: Outcome) -> Self {
        match (opponent, outcome) {
            (Rock, Win) | (Paper, Draw) | (Scissors, Lose) => Paper,
            (Scissors, Win) | (Rock, Draw) | (Paper, Lose) => Rock,
            (Paper, Win) | (Scissors, Draw) | (Rock, Lose) => Scissors,
        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{} is not a valid choice character", *_0 as char)]
pub struct InvalidChoiceCharacter(u8);

impl Outcome {
    pub fn from_char(c: u8) -> Result<Self, InvalidOutcomeCharacter> {
        match c {
            b'X' => Ok(Lose),
            b'Y' => Ok(Draw),
            b'Z' => Ok(Win),
            _ => Err(InvalidOutcomeCharacter(c)),
        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{} is not a valid outcome character", *_0 as char)]
pub struct InvalidOutcomeCharacter(u8);
