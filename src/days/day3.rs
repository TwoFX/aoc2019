use anyhow::{Error, Result};
use std::{
    cmp,
    collections::{HashMap, HashSet},
    num::ParseIntError,
    str::FromStr,
};
use thiserror::Error;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn dx(&self) -> i64 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            Direction::Up => 0,
            Direction::Down => 0,
        }
    }

    fn dy(&self) -> i64 {
        match self {
            Direction::Left => 0,
            Direction::Right => 0,
            Direction::Up => 1,
            Direction::Down => -1,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(ParseError::MalformedDirection),
        }
    }
}

struct Instruction {
    dir: Direction,
    distance: i64,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, rest) = split_first(s).ok_or(ParseError::MalformedInstruction)?;
        let dir: Direction = first.try_into()?;
        let distance: i64 = rest.parse()?;
        Ok(Instruction { dir, distance })
    }
}

fn split_first(s: &str) -> Option<(char, &str)> {
    let mut iter = s.chars();
    let head = iter.next()?;
    Some((head, iter.as_str()))
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Malformed direction")]
    MalformedDirection,
    #[error("Malformed instruction")]
    MalformedInstruction,
    #[error(transparent)]
    MalformedDistance(#[from] ParseIntError),
    #[error("Malformed input")]
    MalformedInput,
}

fn parse_input_line(line: &str) -> Result<Vec<Instruction>, ParseError> {
    line.split(',').map(|s| s.parse::<Instruction>()).collect()
}

fn parse_input(input: &str) -> Result<(Vec<Instruction>, Vec<Instruction>), ParseError> {
    let maybe_lines: Result<Vec<Vec<Instruction>>, _> =
        input.trim().split('\n').map(parse_input_line).collect();
    let lines = maybe_lines?;
    let mut iter = lines.into_iter();
    let first = iter.next().ok_or(ParseError::MalformedInput)?;
    let second = iter.next().ok_or(ParseError::MalformedInput)?;
    if iter.next().is_some() {
        return Err(ParseError::MalformedInput);
    }

    Ok((first, second))
}

fn visited_positions(instructions: &[Instruction]) -> Vec<(i64, i64)> {
    let mut result = vec![(0, 0)];
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    for inst in instructions {
        for _ in 0..inst.distance {
            x += inst.dir.dx();
            y += inst.dir.dy();

            result.push((x, y));
        }
    }

    result
}

fn visited_positions_map(instructions: &[Instruction]) -> HashMap<(i64, i64), i64> {
    let mut result = HashMap::new();
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut t: i64 = 0;

    for inst in instructions {
        for _ in 0..inst.distance {
            x += inst.dir.dx();
            y += inst.dir.dy();
            t += 1;

            result.entry((x, y)).or_insert(t);
        }
    }

    result
}

fn dist((a, b): (i64, i64)) -> i64 {
    a.abs() + b.abs()
}

fn update_best(prev: Option<i64>, d: i64) -> Option<i64> {
    if d == 0 {
        return prev;
    }
    match prev {
        None => Some(d),
        Some(old) => Some(cmp::min(old, d)),
    }
}

pub fn part_a(input: &str) -> Result<String> {
    let (first, second) = parse_input(input)?;

    let first_pos: HashSet<(i64, i64)> = visited_positions(first.as_slice()).into_iter().collect();
    let second_pos = visited_positions(second.as_slice());

    let mut ans: Option<i64> = None;

    for pos in second_pos {
        if first_pos.contains(&pos) {
            ans = update_best(ans, dist(pos));
        }
    }

    let fin = ans.ok_or(Error::msg("No crossings"))?;

    Ok(fin.to_string())
}

pub fn part_b(input: &str) -> Result<String> {
    let (first, second) = parse_input(input)?;

    let first_pos = visited_positions_map(first.as_slice());
    let second_pos = visited_positions_map(second.as_slice());

    let mut ans: Option<i64> = None;

    for (pos, t) in second_pos {
        if let Some(t2) = first_pos.get(&pos) {
            ans = update_best(ans, t + t2);
        }
    }

    let fin = ans.ok_or(Error::msg("No crossings"))?;

    Ok(fin.to_string())
}
