// Advent of Code 2021, Day 6
// https://adventofcode.com/2021/day/6

use cached::proc_macro::cached;
use std::collections::BTreeMap;
use std::{io::BufRead, str::FromStr};

fn next_day_counts(current_state: BTreeMap<usize, usize>) -> BTreeMap<usize, usize> {
    let mut result: BTreeMap<usize, usize> = (0..=8).map(|i| (i, 0)).collect();

    current_state.iter().for_each(|(counter, count)| {
        match counter.checked_sub(1) {
            Some(new_counter) => {
                // Move this count to the new counter
                result.entry(new_counter).and_modify(|c| *c += count);
            }
            None => {
                // Reset the counters
                result.entry(6).and_modify(|c| *c += count);

                // Create new fish
                result.entry(8).and_modify(|c| *c += count);
            }
        };
    });

    result
}

fn counting_solution(fish: &[Lanternfish], days: usize) -> usize {
    // Create a map of counter to num of fish with that counter
    let mut counter_to_count: BTreeMap<usize, usize> = BTreeMap::new();
    fish.iter().for_each(|f| {
        counter_to_count
            .entry(f.counter)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    });

    for _ in 0..days {
        counter_to_count = next_day_counts(counter_to_count);
    }

    counter_to_count.values().sum()
}

/// Calculates how many fish we would end up with after <days> days
/// starting from a fish with the given counter
#[cached]
fn num_fish_after_impl(counter: usize, days: usize) -> usize {
    if days <= counter {
        return 1;
    }

    // Account for the number of fish we will create just by resetting
    // due to the amount of days that we're asked to advance
    let num_children = (days - counter - 1) / 7 + 1;

    // Now for each of these children we calculate how many fish they would have created
    // For this we need to know which day (relative to now) each child was created
    // and advance its counter by <days> - <created_day>
    // Add 1 to account for self
    1 + (0..num_children)
        // Calculate when it would have been created
        .map(|i| 7 * i + counter + 1)
        // Recursion!
        .map(|creation_day| num_fish_after_impl(8, days.saturating_sub(creation_day)))
        // Add it all up
        .sum::<usize>()
}

#[derive(Debug, Clone, Copy)]
struct Lanternfish {
    pub counter: usize,
}

impl Lanternfish {
    pub fn new(counter: usize) -> Self {
        Self { counter }
    }

    /// Calculates how many fish we would end up with after <days> days
    /// starting from this fish
    pub fn num_fish_after(&self, days: usize) -> usize {
        num_fish_after_impl(self.counter, days)
    }
}

fn parse_input_line(s: &str) -> anyhow::Result<Vec<Lanternfish>> {
    Ok(s.split(',')
        .map(|n| usize::from_str(n).map(Lanternfish::new))
        .collect::<Result<_, _>>()?)
}

fn main_impl() -> anyhow::Result<()> {
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line)?;

    let fish = parse_input_line(&line)?;

    println!(
        "Part 1 solution {}",
        fish.iter().map(|f| f.num_fish_after(80)).sum::<usize>()
    );

    println!(
        "Part 1 solution (counting) {}",
        counting_solution(&fish, 80),
    );

    println!(
        "Part 2 solution {}",
        fish.iter().map(|f| f.num_fish_after(256)).sum::<usize>()
    );

    println!(
        "Part 2 solution (counting) {}",
        counting_solution(&fish, 256),
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
        assert_eq!(Lanternfish::new(3).num_fish_after(1), 1);
    }

    #[test]
    fn test_num_fish_after_counter_hits_zero() {
        assert_eq!(Lanternfish::new(3).num_fish_after(3), 1);
    }

    #[test]
    fn test_num_fish_after_counter_passes_zero() {
        assert_eq!(Lanternfish::new(3).num_fish_after(4), 2);
    }
}
