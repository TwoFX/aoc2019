use anyhow::{Error, Result};
use aoc19::days::{day1, day2, day3, day4, day5};
use std::{
    env,
    io::{self, Read},
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).ok_or(Error::msg("Missing day"))?.as_str();
    let part = args.get(2).ok_or(Error::msg("Missing part"))?.as_str();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let inp = input.as_str();

    let ans = match (day, part) {
        ("1", "a") => day1::part_a(inp),
        ("1", "b") => day1::part_b(inp),
        ("2", "a") => day2::part_a(inp),
        ("2", "b") => day2::part_b(inp),
        ("3", "a") => day3::part_a(inp),
        ("3", "b") => day3::part_b(inp),
        ("4", "a") => day4::part_a(inp),
        ("4", "b") => day4::part_b(inp),
        ("5", "a") => day5::part_a(inp),
        ("5", "b") => day5::part_b(inp),
        _ => Err(Error::msg("Unknown day/part combination")),
    }?;

    println!("{ans}");
    Ok(())
}
