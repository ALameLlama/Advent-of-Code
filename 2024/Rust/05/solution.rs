use std::collections::{HashMap, HashSet};

pub fn run(only_part1: bool, only_part2: bool) {
    let input = include_str!("../../input/day05.txt");

    println!("Day 05");
    if !only_part2 {
        println!("Part 1 answer: {}", part1(input));
    }
    if !only_part1 {
        println!("Part 2 answer: {}", part2(input));
    }
}

fn part1(input: &str) -> i64 {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let rules: HashMap<i64, HashSet<i64>> = {
        let mut rules: HashMap<i64, HashSet<i64>> = HashMap::new();

        for line in parts[0].lines() {
            let rule_parts: Vec<i64> = line.split('|').map(|s| s.parse().unwrap()).collect();

            rules
                .entry(rule_parts[0])
                .or_default()
                .insert(rule_parts[1]);
        }

        rules
    };

    let updates: Vec<Vec<i64>> = {
        parts[1]
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
            .collect()
    };

    updates
        .iter()
        .filter(|update| {
            let rules: &HashMap<i64, HashSet<i64>> = &rules;
            let update_numbers: HashSet<i64> = update.iter().copied().collect();

            for (i, &num1) in update.iter().enumerate() {
                for &num2 in &update[i + 1..] {
                    // check if num 2 exists in hash mpa so we if something comes before it
                    if let Some(must_be_before) = rules.get(&num2) {
                        // if num 1 is in must_be_before and num 2 is in update_numbers it comes after
                        if must_be_before.contains(&num1) && update_numbers.contains(&num2) {
                            return false;
                        }
                    }
                }
            }

            true
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part2(input: &str) -> i64 {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let rules: HashMap<i64, HashSet<i64>> = {
        let mut rules: HashMap<i64, HashSet<i64>> = HashMap::new();

        for line in parts[0].lines() {
            let rule_parts: Vec<i64> = line.split('|').map(|s| s.parse().unwrap()).collect();

            rules
                .entry(rule_parts[0])
                .or_default()
                .insert(rule_parts[1]);
        }

        rules
    };

    let updates: Vec<Vec<i64>> = {
        parts[1]
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
            .collect()
    };

    updates
        .iter()
        .filter(|update| {
            let rules: &HashMap<i64, HashSet<i64>> = &rules;
            let update_numbers: HashSet<i64> = update.iter().copied().collect();

            for (i, &num1) in update.iter().enumerate() {
                for &num2 in &update[i + 1..] {
                    // check if num 2 exists in hash mpa so we if something comes before it
                    if let Some(must_be_before) = rules.get(&num2) {
                        // if num 1 is in must_be_before and num 2 is in update_numbers it comes after
                        if must_be_before.contains(&num1) && update_numbers.contains(&num2) {
                            return true;
                        }
                    }
                }
            }

            false
        })
        .map(|update| {
            let mut numbers: Vec<i64> = update.clone();

            // honestly this seems awful. just keep swapping until no more swaps are made.
            // I think this is called bubble sort but nested?
            for i in 0..numbers.len() {
                for j in i + 1..numbers.len() {
                    // If j must come before i, swap them
                    if let Some(must_be_before) = rules.get(&numbers[j]) {
                        if must_be_before.contains(&numbers[i]) {
                            numbers.swap(i, j);
                        }
                    }
                }
            }

            numbers[numbers.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 123);
    }
}

