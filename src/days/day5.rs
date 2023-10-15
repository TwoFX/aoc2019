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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_a() {
        let input = fs::read_to_string("inputs/day5.in").unwrap();
        let expected_output = fs::read_to_string("inputs/day5a.out").unwrap();

        let actual_output = part_a(input.as_str()).unwrap();

        assert_eq!(expected_output.trim(), actual_output.trim());
    }

    #[test]
    fn test_part_b() {
        let input = fs::read_to_string("inputs/day5.in").unwrap();
        let expected_output = fs::read_to_string("inputs/day5b.out").unwrap();

        let actual_output = part_b(input.as_str()).unwrap();

        assert_eq!(expected_output.trim(), actual_output.trim());
    }
}
