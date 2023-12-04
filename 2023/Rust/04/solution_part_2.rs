fn main() {
    let input = include_str!("./input2.txt").lines().collect();

    let output: u32 = aoc_process(input).unwrap_or_default();

    println!("{output}");
}

fn aoc_process(input: Vec<&str>) -> Result<u32, &'static str> {
    let mut unscratched_cards: Vec<String> = Vec::new();
    let mut total_cards = 0;

    // Process original cards.
    for (_i, card) in input.iter().enumerate() {
        process_card(&input, card, &mut total_cards, &mut unscratched_cards);
    }

    // Process new cards.
    while !unscratched_cards.is_empty() {
        let next_card = unscratched_cards.pop().unwrap();
        process_card(&input, &next_card, &mut total_cards, &mut unscratched_cards);
    }

    return Ok(total_cards);
}

fn process_card(
    all_cards: &[&str],
    card: &str,
    total_cards: &mut u32,
    unscratched_cards: &mut Vec<String>,
) {
    let card_parsed: Vec<&str> = card.split(": ").collect();
    *total_cards += 1;

    let card_id: u32 = card_parsed[0]
        .split_whitespace()
        .last()
        .ok_or("Failed to get id")
        .unwrap_or_default()
        .parse::<u32>()
        .unwrap_or_default();

    let scores: Vec<&str> = card_parsed[1].split(" | ").collect();

    let winning_nums: Vec<u32> = scores[0]
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap_or_default())
        .collect();

    let scoring_nums: Vec<u32> = scores[1]
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap_or_default())
        .collect();

    let hits = scoring_nums
        .iter()
        .filter(|&x| winning_nums.iter().any(|y| y == x))
        .count();

    for i in 0..hits {
        let next_card_id: u32 = card_id + i as u32;
        if let Some(next_card) = all_cards.get(next_card_id as usize) {
            unscratched_cards.push(next_card.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "Card   1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .lines()
            .collect();

        let output: u32 = aoc_process(input).unwrap_or_default();

        assert_eq!(output, 30);
    }
}