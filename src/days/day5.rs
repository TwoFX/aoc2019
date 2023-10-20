use anyhow::{Error, Result};

use crate::intcode::Program;

pub fn part_a(input: &str) -> Result<String> {
    let mut p: Program = input.trim().parse()?;
    let output = p.execute(&[1])?;
    let ans = *output.last().ok_or(Error::msg("Empty output"))?;
    Ok(format!("{ans}"))
}

pub fn part_b(input: &str) -> Result<String> {
    let mut p: Program = input.trim().parse()?;
    let output = p.execute(&[5])?;
    let ans = *output.last().ok_or(Error::msg("Empty output"))?;
    Ok(format!("{ans}"))
}
