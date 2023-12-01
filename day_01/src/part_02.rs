use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    Ok(input
        .iter()
        .map(|row| {
            let digits: Vec<u32> = row
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if c.is_ascii_digit() {
                        return Some(c.to_digit(10).unwrap());
                    }
                    let chars = row.chars().skip(i).take(5).collect::<String>();

                    if chars.starts_with("one") {
                        return Some(1);
                    }

                    if chars.starts_with("two") {
                        return Some(2);
                    }

                    if chars.starts_with("three") {
                        return Some(3);
                    }

                    if chars.starts_with("four") {
                        return Some(4);
                    }

                    if chars.starts_with("five") {
                        return Some(5);
                    }

                    if chars.starts_with("six") {
                        return Some(6);
                    }

                    if chars.starts_with("seven") {
                        return Some(7);
                    }

                    if chars.starts_with("eight") {
                        return Some(8);
                    }

                    if chars.starts_with("nine") {
                        return Some(9);
                    }

                    None
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
        .sum())
}