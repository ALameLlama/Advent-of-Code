pub fn run(only_part1: bool, only_part2: bool) {
    let input = include_str!("../../input/day02.txt");

    println!("Day 02");
    if !only_part2 {
        println!("Part 1 answer: {}", part1(input));
    }
    if !only_part1 {
        println!("Part 2 answer: {}", part2(input));
    }
}

fn part1(input: &str) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        let mut is_safe = true;
        let is_increasing = numbers[0] < numbers[1];
        for i in 0..numbers.len() - 1 {
            let diff = (numbers[i] - numbers[i + 1]).abs();

            // same as diff < 1 || diff > 3
            if !(1..4).contains(&diff) {
                is_safe = false;
                break;
            }

            if (is_increasing && numbers[i] > numbers[i + 1])
                || (!is_increasing && numbers[i] < numbers[i + 1])
            {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            sum += 1;
        }
    }

    sum
}

fn part2(input: &str) -> i64 {
    let mut sum = 0;
    'outer: for line in input.lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        if is_safe_sequence(&numbers) {
            sum += 1;
            continue;
        }

        for skip_index in 0..numbers.len() {
            let modified_numbers: Vec<i64> = numbers
                .iter()
                .enumerate() // give indexes
                .filter(|&(i, _)| i != skip_index) // remove current index
                .map(|(_, &x)| x) // map back to values, idk if this is the easiest way but it works lol
                .collect();

            if is_safe_sequence(&modified_numbers) {
                sum += 1;
                continue 'outer;
            }
        }
    }
    sum
}

fn is_safe_sequence(numbers: &[i64]) -> bool {
    let is_increasing = numbers[0] < numbers[1];

    for i in 0..numbers.len() - 1 {
        let diff = (numbers[i] - numbers[i + 1]).abs();
        // same as diff < 1 || diff > 3
        if !(1..4).contains(&diff) {
            return false;
        }

        if (is_increasing && numbers[i] > numbers[i + 1])
            || (!is_increasing && numbers[i] < numbers[i + 1])
        {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
    }

    #[test]
    fn test_part2_edge_case_1() {
        assert_eq!(part2("66 67 68 71 72 69"), 1);
    }
}
