use std::collections::HashMap;
use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<i32> {
    Ok(input
        .iter()
        .map(|game| {
            let mut res_map: HashMap<&str, i32> = HashMap::new();
            let parts: Vec<&str> = game.split(':').collect();
            parts[1]
                .split(';')
                .for_each(|draw| {
                    draw.split(',').for_each(|dice| {
                        let parts: Vec<&str> = dice.trim().split(' ').collect();
                        let colour_value = res_map.entry(parts[1]).or_insert(0);
                        if *colour_value < parts[0].parse::<i32>().unwrap() {
                            res_map.insert(parts[1], parts[0].parse::<i32>().unwrap());
                        }
                    });

                });

            res_map.iter().fold(1, |acc, (_key, val)| acc * val)
        })
        .sum()
    )
}