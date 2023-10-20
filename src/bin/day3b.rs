use anyhow::Result;
use aoc19::days::day3;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let ans = day3::part_b(input.as_str())?;
    println!("{ans}");
    Ok(())
}
