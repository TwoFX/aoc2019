use anyhow::{Error, Result};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Digit {
    Zero,
    One,
    Two,
}

struct Layer {
    content: Vec<Vec<Digit>>,
}

impl Layer {
    fn hist(&self) -> HashMap<Digit, usize> {
        let mut res = HashMap::new();

        for row in &self.content {
            for dig in row {
                *res.entry(*dig).or_insert(0) += 1;
            }
        }

        res
    }
}

fn parse(s: &str) -> Result<Vec<Layer>> {
    let mut remaining = s;

    let num_layers = s.len() / (25 * 6);
    let mut res = Vec::with_capacity(num_layers);
    for _ in 0..num_layers {
        let mut layer_content = Vec::with_capacity(6);
        for _ in 0..6 {
            let (head, tail) = remaining.split_at(25);
            remaining = tail;
            let maybe_row: Result<Vec<Digit>> = head
                .chars()
                .map(|d| match d {
                    '0' => Ok(Digit::Zero),
                    '1' => Ok(Digit::One),
                    '2' => Ok(Digit::Two),
                    _ => Err(Error::msg("Unknown digit")),
                })
                .collect();
            layer_content.push(maybe_row?);
        }
        res.push(Layer {
            content: layer_content,
        });
    }

    Ok(res)
}

pub fn part_a(input: &str) -> Result<String> {
    let image = parse(input)?;

    let mut best = None;
    for layer in &image {
        let hist = layer.hist();
        let have = (
            hist.get(&Digit::Zero).copied().unwrap_or(0),
            hist.get(&Digit::One).copied().unwrap_or(0)
                * hist.get(&Digit::Two).copied().unwrap_or(0),
        );

        best = match best {
            None => Some(have),
            o @ Some((zeros, _)) => {
                if have.0 < zeros {
                    Some(have)
                } else {
                    o
                }
            }
        }
    }

    Ok(best.ok_or(Error::msg("No layers"))?.1.to_string())
}

#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
}

pub fn part_b(input: &str) -> Result<String> {
    let image = parse(input)?;
    let mut res = [[None; 25]; 6];

    for layer in &image {
        for r in 0..6 {
            for c in 0..25 {
                res[r][c] = res[r][c].or_else(|| match layer.content[r][c] {
                    Digit::Zero => Some(Color::Black),
                    Digit::One => Some(Color::White),
                    Digit::Two => None,
                });
            }
        }
    }

    let mut ans: String = String::new();
    for r in 0..6 {
        for c in 0..25 {
            match res[r][c].ok_or(Error::msg("Empty pixel"))? {
                Color::Black => ans.push(' '),
                Color::White => ans.push('#'),
            }
        }
        ans.push('\n');
    }

    Ok(ans)
}
