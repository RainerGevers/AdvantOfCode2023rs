use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    Ok(input
        .iter()
        .map(|row| {
            let digits: Vec<u32> = row
                .chars()
                .map(|c| {
                    if c.is_ascii_digit() {
                        Some(c.to_digit(10).unwrap())
                    } else {
                        None
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