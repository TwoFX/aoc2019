use crate::intcode::Program;
use anyhow::{Error, Result};

pub fn part_a(input: &str) -> Result<String> {
    let mut p: Program = input.trim().parse()?;
    p.code[1] = 12;
    p.code[2] = 2;
    p.execute(&[])?;
    let ans = p.code[0];
    Ok(format!("{ans}"))
}

pub fn part_b(input: &str) -> Result<String> {
    let p: Program = input.trim().parse()?;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut q = p.clone();
            q.code[1] = noun;
            q.code[2] = verb;
            if q.execute(&[]).is_ok() && q.code[0] == 19_690_720 {
                let ans = 100 * noun + verb;
                return Ok(format!("{ans}"));
            }
        }
    }
    Err(Error::msg("Did not find a solution"))
}
