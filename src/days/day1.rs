use anyhow::Result;

pub fn part_a(input: &str) -> Result<String> {
    let stuff: Vec<i32> = input
        .trim()
        .split('\n')
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()?;

    let ans: i32 = stuff.iter().map(|x| x / 3 - 2).sum();
    Ok(ans.to_string())
}

fn solve(x: i32) -> i32 {
    let fuel = x / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + solve(fuel)
    }
}

pub fn part_b(input: &str) -> Result<String> {
    let stuff: Vec<i32> = input
        .trim()
        .split('\n')
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()?;

    let ans: i32 = stuff.iter().map(|x| solve(*x)).sum();
    Ok(ans.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_a() {
        let input = fs::read_to_string("inputs/day1.in").unwrap();
        let expected_output = fs::read_to_string("inputs/day1a.out").unwrap();

        let actual_output = part_a(input.as_str()).unwrap();

        assert_eq!(expected_output.trim(), actual_output.trim());
    }

    #[test]
    fn test_part_b() {
        let input = fs::read_to_string("inputs/day1.in").unwrap();
        let expected_output = fs::read_to_string("inputs/day1b.out").unwrap();

        let actual_output = part_b(input.as_str()).unwrap();

        assert_eq!(expected_output.trim(), actual_output.trim());
    }
}
