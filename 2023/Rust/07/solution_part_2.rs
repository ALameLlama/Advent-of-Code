use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2.txt");

    let output: u64 = aoc_process(input).unwrap_or_default();

    println!("{output}");
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, PartialEq, Eq)]
struct Hands {
    cards: Value,
    hand_type: HandType,
    bet: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Value {
    hand: String,
    score: Vec<u32>,
}

impl Ord for Hands {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare hand_type first
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                // If hand_type is equal, compare scoresm by iter so it compares down the whole vec
                // self.cards.score.cmp(&other.cards.score)
                self.cards.score.iter().cmp(other.cards.score.iter())
            }
            // If they're not eq return ordering result, idk how this works really but google says
            // it works.
            ordering => ordering,
        }
    }
}

// Use the existing Ord implementation to obtain the ordering.
impl PartialOrd for Hands {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn aoc_process(input: &str) -> Result<u64, &'static str> {
    use HandType::*;

    let mut hands: Vec<Hands> = input
        .lines()
        .into_iter()
        .map(|line| {
            let (cards, bet) = line.split_once(" ").unwrap();

            // Replace 'J' with the card that it pretends to be
            let faked_cards = cards.replace('J', &best_pretend_card(&cards).to_string().as_str());

            // iter over all the chars and count how many of each.
            // E.G
            // 32T3K|32T3K = 1112
            // T55J5|T5555 = 14
            // KK677|KK677 = 122
            // KTJJT|KTTTT = 14
            //  QQQJA|QQQQA = 14
            let values_str: String = {
                let mut values: Vec<_> = faked_cards
                    .chars()
                    .fold(HashMap::new(), |mut counts, c| {
                        *counts.entry(c).or_insert(0) += 1;
                        counts
                    })
                    .values()
                    .cloned()
                    .collect();

                values.sort();
                values.into_iter().map(|v| v.to_string()).collect()
            };

            let hand_type = match values_str.as_str() {
                "5" => FiveOfAKind,
                "14" => FourOfAKind,
                "23" => FullHouse,
                "113" => ThreeOfAKind,
                "122" => TwoPair,
                "1112" => OnePair,
                "11111" => HighCard,
                _ => panic!("Failed to get a hand type??"),
            };

            let score: Vec<u32> = cards
                .chars()
                .map(|card| match card {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 1,
                    'T' => 10,
                    value => value.to_digit(10).unwrap(),
                })
                .collect();

            return Hands {
                cards: Value {
                    hand: cards.to_string(),
                    score,
                },
                hand_type,
                bet: bet.parse().unwrap(),
            };
        })
        .collect();

    hands.sort();

    let total: u32 = hands
        .iter_mut()
        .enumerate()
        .map(|(index, hand)| (index + 1) as u32 * hand.bet)
        .sum();

    return Ok(total as u64);
}

fn best_pretend_card(cards: &str) -> char {
    // Count occurrences of each non-J card
    let counts: HashMap<char, usize> =
        cards
            .chars()
            .filter(|&c| c != 'J')
            .fold(HashMap::new(), |mut counts, c| {
                *counts.entry(c).or_insert(0) += 1;
                counts
            });

    // Find the non-J card with the maximum occurrences
    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(card, _)| card)
        .unwrap_or('J')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let output: u64 = aoc_process(input).unwrap_or_default();

        assert_eq!(output, 5905);
    }
}