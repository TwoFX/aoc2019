use anyhow::Result;
use aoc19::days::day2;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let ans = day2::part_a(input.as_str())?;
    println!("{ans}");
    Ok(())
}
