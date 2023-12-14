use std::{
    fmt::Display,
    process::{Command, Output, Stdio},
};

use crate::template::day::Day;

#[derive(Debug)]
pub enum AoCCommandError {
    CommandNotFound,
    CommandNotCallable,
    BadExitStatus(Output),
}

impl Display for AoCCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AoCCommandError::CommandNotFound => write!(f, "aoc-cli is not present in environment."),
            AoCCommandError::CommandNotCallable => write!(f, "aoc-cli could not be called."),
            AoCCommandError::BadExitStatus(_) => {
                write!(f, "aoc-cli exited with a non-zero status.")
            }
        }
    }
}

pub fn check() -> Result<(), AoCCommandError> {
    Command::new("aoc")
        .arg("-V")
        .output()
        .map_err(|_| AoCCommandError::CommandNotFound)?;
    Ok(())
}

pub fn read(day: Day) -> Result<Output, AoCCommandError> {
    let puzzle_path = get_puzzle_path(day);

    let args = build_args(
        "read",
        &[
            "--description-only".into(),
            "--puzzle-file".into(),
            puzzle_path,
        ],
        day,
    );

    call_aoc_cli(&args)
}

pub fn download(day: Day) -> Result<Output, AoCCommandError> {
    let input_path = get_input_path(day);
    let puzzle_path = get_puzzle_path(day);

    let args = build_args(
        "download",
        &[
            "--overwrite".into(),
            "--input-file".into(),
            input_path.to_string(),
            "--puzzle-file".into(),
            puzzle_path.to_string(),
        ],
        day,
    );

    let output = call_aoc_cli(&args)?;
    println!("--");
    print!(" Successfully wrote input to \"{}\".", &input_path);
    print!(" Successfully wrote puzzle to \"{}\".", &puzzle_path);

    Ok(output)
}

pub fn submit(day: Day, part: u8, result: &str) -> Result<Output, AoCCommandError> {
    let mut args = build_args("submit", &[], day);
    args.push(part.to_string());
    args.push(result.to_string());
    call_aoc_cli(&args)
}

fn get_input_path(day: Day) -> String {
    format!("data/inputs/{day}.txt")
}

fn get_puzzle_path(day: Day) -> String {
    format!("data/puzzles/{day}.txt")
}

fn get_year() -> Option<u16> {
    match std::env::var("AOC_YEAR") {
        Ok(x) => x.parse().ok().or(None),
        Err(_) => None,
    }
}

fn build_args(command: &str, args: &[String], day: Day) -> Vec<String> {
    let mut cmd_args = args.to_vec();

    if let Some(year) = get_year() {
        cmd_args.push("--year".into());
        cmd_args.push(year.to_string());
    }

    cmd_args.append(&mut vec!["--day".into(), day.to_string(), command.into()]);

    cmd_args
}

fn call_aoc_cli(args: &[String]) -> Result<Output, AoCCommandError> {
    let output = Command::new("aoc")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|_| AoCCommandError::CommandNotCallable)?;

    if output.status.success() {
        Ok(output)
    } else {
        Err(AoCCommandError::BadExitStatus(output))
    }
}
