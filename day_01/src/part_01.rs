use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    Ok(input
        .into_iter()
        .map(|row| {
            let digits: Vec<u32> = row
                .chars()
                .into_iter()
                .map(|c| {
                    if c.is_digit(10) {
                        return Some(c.to_digit(10).unwrap())
                    } else {
                        return None
                    }
                })
                .filter(|o| match o {
                    Some(_val) => true,
                    None => false,
                })
                .map(|val| val.unwrap())
                .collect::<Vec<u32>>();
            let first = digits.first().unwrap_or(&0).to_owned();
            let last = digits.last().unwrap_or(&0).to_owned();
            format!("{}{}", first, last).as_str().parse().unwrap_or(0)
        })
        .sum()
    )
}