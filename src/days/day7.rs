use std::cmp;

use crate::intcode::{Program, ProgramState};
use anyhow::{Error, Result};

fn find_longest_decreasing_suffix<T: Ord>(s: &[T]) -> Option<usize> {
    for i in (0..s.len() - 1).rev() {
        if s[i] < s[i + 1] {
            return Some(i);
        }
    }

    None
}

fn next_permutation<T: Ord>(s: &mut [T]) -> bool {
    let Some(i) = find_longest_decreasing_suffix(s) else {
        return false;
    };

    let mut swap_target = i + 1;
    for j in i + 1..s.len() {
        if s[i] < s[j] && s[j] < s[swap_target] {
            swap_target = j
        }
    }

    s.swap(i, swap_target);
    s[i + 1..].reverse();

    true
}

pub fn part_a(input: &str) -> Result<String> {
    let p: Program = input.trim().parse()?;

    let mut v = vec![0, 1, 2, 3, 4];

    let mut best = None;
    loop {
        let mut last = 0;
        for &phase in &v {
            let mut q = p.clone();
            let (_, outp) = q.execute(&[phase, last])?;
            last = *outp.get(0).ok_or(Error::msg("Missing output"))?;
        }

        best = match best {
            None => Some(last),
            Some(prev) => Some(cmp::max(prev, last)),
        };

        if !next_permutation(v.as_mut_slice()) {
            break;
        }
    }

    let ans = best.unwrap();
    Ok(ans.to_string())
}

pub fn part_b(input: &str) -> Result<String> {
    let p: Program = input.trim().parse()?;

    let mut v = vec![5, 6, 7, 8, 9];

    let mut best = None;
    loop {
        let mut qs = Vec::new();
        for &phase in &v {
            let mut q = p.clone();
            let (ProgramState::ExpectingInput, _) = q.execute(&[phase])? else {
                return Err(Error::msg("Early halt"));
            };
            qs.push(q);
        }

        let mut last = 0;
        loop {
            let mut last_state = None;
            for q in &mut qs {
                let (res, outp) = q.execute(&[last])?;
                last_state = Some(res);
                last = *outp.get(0).ok_or(Error::msg("Missing output"))?;
            }
            if last_state.is_some_and(|s| s == ProgramState::Exited) {
                break;
            }
        }

        best = match best {
            None => Some(last),
            Some(prev) => Some(cmp::max(prev, last)),
        };

        if !next_permutation(v.as_mut_slice()) {
            break;
        }
    }

    let ans = best.unwrap();
    Ok(ans.to_string())
}
