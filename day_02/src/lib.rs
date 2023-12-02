pub mod part_01;
pub mod part_02;

pub type Input = Vec<String>;

pub fn parse_input(input: &str) -> Input {
    input
        .trim_start()
        .trim_end()
        .lines()
        .map(str::trim)
        .map(|string| string.parse().unwrap())
        .collect()
}