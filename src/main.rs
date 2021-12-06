// Advent of Code 2021, Day 6
// https://adventofcode.com/2021/day/6

use std::{io::BufRead, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct LanternFish {
    counter: usize,
}

impl LanternFish {
    pub fn new(counter: usize) -> Self {
        Self { counter }
    }
}

fn parse_input_line(s: &str) -> anyhow::Result<Vec<LanternFish>> {
    Ok(s.split(',')
        .map(|n| usize::from_str(n).map(LanternFish::new))
        .collect::<Result<_, _>>()?)
}

fn main_impl() -> anyhow::Result<()> {
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line)?;

    let fish = parse_input_line(&line);
    println!("Parsed input {:#?}", fish);

    Ok(())
}

fn main() {
    main_impl().unwrap();
}
