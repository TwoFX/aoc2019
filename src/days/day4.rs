use anyhow::{Error, Result};

mod parta {
    fn good_impl<T: Ord>(s: &[T]) -> bool {
        let mut double = false;
        let mut cur_group = 1;
        for a in s.windows(2) {
            let [l, r] = a else {
                panic!("Unexpected structure")
            };

            if l > r {
                return false;
            }

            if l == r {
                cur_group += 1;
                if cur_group == 2 {
                    double = true;
                }
            } else {
                cur_group = 1;
            }
        }
        double
    }

    pub fn good(n: i32) -> bool {
        let s: Vec<char> = n.to_string().chars().collect();
        good_impl(s.as_slice())
    }
}

mod partb {
    fn good_impl<T: Ord>(s: &[T]) -> bool {
        let mut double = false;
        let mut cur_group = 1;
        for a in s.windows(2) {
            let [l, r] = a else {
                panic!("Unexpected structure")
            };

            if l > r {
                return false;
            }

            if l == r {
                cur_group += 1;
            } else {
                if cur_group == 2 {
                    double = true;
                }
                cur_group = 1;
            }
        }
        cur_group == 2 || double
    }

    pub fn good(n: i32) -> bool {
        let s: Vec<char> = n.to_string().chars().collect();
        good_impl(s.as_slice())
    }
}

fn parse(input: &str) -> Result<(i32, i32)> {
    let values = input
        .trim()
        .split('-')
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    let [l, r] = values.as_slice() else {
        return Err(Error::msg("Invalid input"));
    };
    Ok((*l, *r))
}

pub fn part_a(input: &str) -> Result<String> {
    let (l, r) = parse(input)?;

    let mut ans = 0;
    for n in l..=r {
        if parta::good(n) {
            ans += 1;
        }
    }
    Ok(format!("{ans}"))
}

pub fn part_b(input: &str) -> Result<String> {
    let (l, r) = parse(input)?;

    let mut ans = 0;
    for n in l..=r {
        if partb::good(n) {
            ans += 1;
        }
    }
    Ok(format!("{ans}"))
}
