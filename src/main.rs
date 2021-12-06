// Advent of Code 2021, Day 6
// https://adventofcode.com/2021/day/6

use std::collections::BTreeMap;
use std::{io::BufRead, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct LanternFish {
    counter: usize,
}

impl LanternFish {
    pub fn new(counter: usize) -> Self {
        Self { counter }
    }

    /// Calculates how many fish we would end up with after <days> days
    /// starting from this fish
    pub fn num_fish_after(&self, days: usize) -> usize {
        self.num_fish_after_impl(days, &mut BTreeMap::new())
    }

    /// Calculates how many fish we would end up with after <days> days
    /// starting from this fish
    fn num_fish_after_impl(&self, days: usize, memo: &mut BTreeMap<(usize, usize), usize>) -> usize {
        if days <= self.counter {
            return 1;
        }

        if let Some(result) = memo.get(&(self.counter, days)) {
            return *result;
        }

        // Account for the number of fish we will create just by resetting
        // due to the amount of days that we're asked to advance
        let num_children = (days - self.counter - 1) / 7 + 1;

        // Now for each of these children we calculate how many fish they would have created
        // For this we need to know which day (relative to now) each child was created
        // and advance its counter by <days> - <created_day>
        // Add 1 to account for self
        1 + (0..num_children)
            // Calculate when it would have been created
            .map(|i| 7 * i + self.counter + 1)
            // Recursion!
            .map(|creation_day| {
                let new_counter = 8;
                let new_days = days.saturating_sub(creation_day);
                let result = LanternFish::new(new_counter)
                    .num_fish_after_impl(new_days, memo);
                // Only memoize things that will need recursion
                if new_days > new_counter {
                    memo.entry((new_counter, new_days)).or_insert(result);
                }
                result
            })
            // Add it all up
            .fold(0, |sum, num_children| sum + num_children)
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

    let fish = parse_input_line(&line)?;

    println!(
        "Part 1 solution {}",
        fish.iter()
            .map(|f| f.num_fish_after(80))
            .fold(0, |sum, fish_count| sum + fish_count),
    );

    println!(
        "Part 2 solution {}",
        fish.iter()
            .map(|f| f.num_fish_after(256))
            .fold(0, |sum, fish_count| sum + fish_count),
    );

    Ok(())
}

fn main() {
    main_impl().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_fish_after() {
        assert_eq!(
            LanternFish::new(3).num_fish_after(1),
            1
        );
    }

    #[test]
    fn test_num_fish_after_counter_hits_zero() {
        assert_eq!(
            LanternFish::new(3).num_fish_after(3),
            1
        );
    }

    #[test]
    fn test_num_fish_after_counter_passes_zero() {
        assert_eq!(
            LanternFish::new(3).num_fish_after(4),
            2
        );
    }
}
