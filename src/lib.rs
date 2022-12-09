use std::{
    fmt::{Debug, Display},
    fs,
    io::ErrorKind,
    time::Instant,
};

use anyhow::{Context, Result};

/// A solution to an AoC problem.
pub trait Solution {
    /// The [`Day`] this solution is for.
    const DAY: Day;
    /// The value type returned from part 1.
    type Output1: Debug + Display;
    /// The value type returned from part 2.
    type Output2: Debug + Display;
    /// Run the solution.
    fn solve(input: &str) -> Result<(Self::Output1, Self::Output2)>;
}

/// Sets the caching strategy for retreiving AoC input.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InputCaching {
    /// Read the cache file if present; otherwise, create it.
    Enabled,
    /// Do not read, write, or create a cache file.
    Disabled,
    /// Do not read the cache file, but still do create or update the cache file.
    UpdateOnly,
}
/// An AoC day number. Must be 1 through 25, inclusive.
#[derive(Copy, Clone, Debug, Eq, PartialEq, derive_more::Display)]
pub struct Day(u8);
impl Day {
    /// Creates a new `Day` object. Fails if the number is not a valid AoC day number.
    pub const fn number(n: u8) -> Result<Self, InvalidDay> {
        if n >= 1 && n <= 25 {
            Ok(Self(n))
        } else {
            Err(InvalidDay(n))
        }
    }
    /// Returns the day number as an unsigned integer.
    pub fn get(self) -> u8 {
        self.0
    }
}
#[derive(Debug, thiserror::Error)]
#[error("There is no day {_0} in AoC")]
pub struct InvalidDay(u8);

/// Retrieve AoC input for the given [`Day`], using the given [`InputCaching`] strategy.
pub fn retrieve_input(day: Day, caching: InputCaching) -> Result<String> {
    let cached_path = format!("input/day{day:02}");

    if caching == InputCaching::Enabled {
        match fs::read_to_string(&cached_path) {
            Ok(data) => return Ok(data),
            Err(err) if err.kind() != ErrorKind::NotFound => {
                eprintln!("ERROR: Failed to read input cache file with error {err}")
            }
            Err(_) => (),
        }
    }

    let session_cookie_value = std::env::var("AOC_SESSION_COOKIE")
        .context("failed to read AOC_SESSION_COOKIE from environment")?;

    let url = format!("https://adventofcode.com/2022/day/{}/input", day);
    let data = ureq::get(&url)
        .set("Cookie", &format!("session={}", session_cookie_value))
        .set("User-Agent", "https://github.com/TyPR124/aoc2022")
        .call()
        .context("http request error")?
        .into_string()
        .context("http response error")?;

    if caching != InputCaching::Disabled {
        if let Err(err) = fs::create_dir_all("input").and_then(|_| fs::write(&cached_path, &data)) {
            eprintln!("ERROR: Failed to cache input for day {day} with error {err}")
        }
    }
    Ok(data)
}

pub fn run_solution<S>() -> Result<()>
where
    S: Solution,
{
    // Get the input
    let day = S::DAY;
    dotenv::dotenv()?;
    let input = retrieve_input(S::DAY, InputCaching::Enabled)?;
    let start = Instant::now();
    let (answer1, answer2) = S::solve(&input)?;
    let time_taken = start.elapsed();
    let seconds_taken = time_taken.as_secs_f64();
    let ms_taken = time_taken.as_millis();
    let us_taken = time_taken.as_micros();
    let debug_or_release = if cfg!(debug_assertions) {
        "DEBUG"
    } else {
        "RELEASE"
    };

    println!("AoC 2022 Day {day:02} Results:");
    println!("------------------------");
    println!("Part 1 Answer: {answer1}");
    println!("Part 2 Answer: {answer2}");
    println!("------------------------");
    println!("Compiled in {debug_or_release} mode");
    if ms_taken > 1000 {
        println!("Time taken: {seconds_taken:.03} s");
    } else if ms_taken > 0 {
        println!("Time taken: {ms_taken} ms");
    } else {
        println!("Time taken: {us_taken} μs");
    }
    Ok(())
}

pub fn test_solution<S>()
where
    S: Solution,
{
    let day = S::DAY;
    let day_prefix = format!("day{day:02}");
    let dir = fs::read_dir("test_input").expect("test_input folder missing");
    for item in dir {
        let item = item.unwrap();
        let meta = item.metadata().unwrap();
        if !meta.is_file() {
            continue;
        }
        let input_path = item.path();
        let input_filename = input_path.file_name().unwrap().to_string_lossy();
        if !(input_filename.ends_with(".input") && input_filename.starts_with(&*day_prefix)) {
            continue;
        }
        let input = fs::read_to_string(&input_path).expect("failed to read test input file");
        let answers_path = input_path.with_extension("answers");
        if !answers_path.exists() {
            panic!(
                "Answers file {} not found",
                answers_path.file_name().unwrap().to_string_lossy()
            )
        }
        let answers = fs::read_to_string(&answers_path).expect("failed to read answers file");
        let mut answers_lines = answers.lines();
        let answer1 = answers_lines.next().expect("missing answer 1");
        let answer2 = answers_lines.next().expect("missing answer 2");

        let (solved1, solved2) = S::solve(&input).expect("solution failed");

        assert_eq!(
            answer1,
            format!("{solved1}"),
            "test {} failed part 1",
            input_path.file_name().unwrap().to_string_lossy()
        );
        assert_eq!(
            answer2,
            format!("{solved2}"),
            "test {} failed part 2",
            input_path.file_name().unwrap().to_string_lossy()
        );

        println!("Test passed: {}", input_filename);
    }
}
