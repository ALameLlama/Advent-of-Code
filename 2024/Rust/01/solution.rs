pub fn run(only_part1: bool, only_part2: bool) {
    let input = include_str!("../../input/day01.txt");
    
    println!("Day 01");
    if !only_part2 {
        println!("Part 1 answer: {}", part1(input));
    }
    if !only_part1 {
        println!("Part 2 answer: {}", part2(input));
    }
}

fn part1(input: &str) -> i64 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i64> = line.split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        if numbers.len() < 2 {
            continue;
        }

        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn part2(input: &str) -> i64 {
    let mut left = Vec::new();
    let mut right = std::collections::HashMap::new();

    for line in input.lines() {
        let numbers: Vec<i64> = line.split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        if numbers.len() < 2 {
            continue;
        }

        left.push(numbers[0]);
        *right.entry(numbers[1]).or_insert(0) += 1;
    }

    left.iter()
        .map(|n| n * right.get(n).copied().unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 31);
    }
}
