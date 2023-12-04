fn main() {
    let input = include_str!("./input1.txt");

    let output: u32 = input
        .lines()
        .map(|line| aoc_process(line).unwrap_or_default())
        .sum();

    println!("{output}");
}

fn aoc_process(input: &str) -> Result<u32, &'static str> {
    let card: Vec<&str> = input
        .split(": ")
        .last()
        .ok_or("Failed to get card")?
        .split(" | ")
        .collect();

    let winning_nums: Vec<u32> = card[0]
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap_or_default())
        .collect();

    let scoring_nums: Vec<u32> = card[1]
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap_or_default())
        .collect();

    let hits = scoring_nums
        .iter()
        .filter(|&x| winning_nums.iter().any(|y| y == x))
        .count();

    let mut score = 0;

    if hits > 0 {
        score = (0..hits - 1).fold(1, |acc, _| acc * 2);
    }

    return Ok(score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let output: u32 = input
            .lines()
            .map(|line| aoc_process(line).unwrap_or_default())
            .sum();

        assert_eq!(output, 13);
    }
}