use jeebs::{result::*, error::AsError};
use std::env;
use thiserror::Error;

fn main() {
    let share = get_share().handle();
    println!("The pirate share is {}", share);
}

fn get_share() -> Result<u64> {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let total = args
        .get(0)
        .ok_or(Errors::UnableToGetFirstArgument)?
        .parse::<u64>()
        .map_err(|_| Errors::UnableToParseFirstArgument)?;
    let crew_size = args
        .get(1)
        .ok_or("Something hasn't worked here".as_error())?
        .parse::<usize>()?;
    Ok(pirate_share(total, crew_size))
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}

#[derive(Error, Debug)]
enum Errors {
    #[error("Unable to get first argument")]
    UnableToGetFirstArgument,
    #[error("The first argument is not a valid number")]
    UnableToParseFirstArgument
}

#[test]
fn returns_correct_share() {
    let share = pirate_share(10, 2);
    assert_eq!(2, share);
}

#[test]
fn panics_when_no_crew() {
    pirate_share(10, 0);
}
