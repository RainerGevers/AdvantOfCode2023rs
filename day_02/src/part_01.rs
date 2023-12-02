use std::collections::HashMap;
use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<i32> {
    let mut colour_hash = HashMap::new();
    colour_hash.insert("blue", 14);
    colour_hash.insert("green", 13);
    colour_hash.insert("red", 12);

    Ok(input
        .iter()
        .enumerate()
        .map(|(i, game)| {
            let parts: Vec<&str> = game.split(':').collect();
            let res: Vec<bool> = parts[1]
                .split(';')
                .map(|draw| {
                    let mut invalid: bool = false;
                    draw.split(',').for_each(|dice| {
                        let parts: Vec<&str> = dice.trim().split(' ').collect();
                        let colour_value = colour_hash.get(parts[1]).unwrap();
                        if *colour_value < parts[0].parse::<i32>().unwrap() {
                            invalid = true
                        }
                    });
                    invalid
                })
                .filter(|val|{
                    *val
                } )
                .collect();

            if res.is_empty() {
                let ans = i + 1;
                return ans as i32;
            }

            0i32
        })
        .sum())
}
