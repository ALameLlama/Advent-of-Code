use regex::Regex;
use std::collections::HashMap;

const MAX_SCORES: &[(&str, u32)] = &[("red", 12), ("green", 13), ("blue", 14)];

fn main() {
    let input = include_str!("./input1.txt");

    let game_regex =
        Regex::new(r"Game (?P<id>\d+): (?P<matches>.*)").expect("Failed to compile regex");

    let score_regex =
        Regex::new(r"(?P<score>\d+) (?P<type>blue|red|green)").expect("Failed to compile regex");

    let max_scores_map: HashMap<_, _> = MAX_SCORES.iter().cloned().collect();

    let output: u32 = input
        .lines()
        .map(|line| {
            aoc_process(line, &game_regex, &score_regex, &max_scores_map).unwrap_or_default()
        })
        .sum();

    println!("{output}");
}

fn aoc_process(
    input: &str,
    game_regex: &Regex,
    score_regex: &Regex,
    max_scores_map: &HashMap<&str, u32>,
) -> Result<u32, &'static str> {
    let game_cap = game_regex.captures(input).ok_or("Failed to get game id")?;

    let id: u32 = game_cap["id"]
        .parse::<u32>()
        .map_err(|_| "Failed to parse game id")?;

    for (_i, match_set) in game_cap["matches"].split(";").enumerate() {
        for score_cap in score_regex.captures_iter(&match_set) {
            if let (Some(type_match), Some(score_match)) =
                (score_cap.name("type"), score_cap.name("score"))
            {
                let type_str = type_match.as_str();
                let score_str = score_match.as_str();

                let score = score_str
                    .parse::<u32>()
                    .map_err(|_| "Failed to get score")?;

                let max_score = max_scores_map.get(type_str).unwrap();

                if &score > max_score {
                    return Err("Over");
                }
            }
        }
    }

    return Ok(id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

        let game_regex =
            Regex::new(r"Game (?P<id>\d+): (?P<matches>.*)").expect("Failed to compile regex");

        let score_regex = Regex::new(r"(?P<score>\d+) (?P<type>blue|red|green)")
            .expect("Failed to compile regex");

        let max_scores_map: HashMap<_, _> = MAX_SCORES.iter().cloned().collect();

        let output: u32 = input
            .lines()
            .map(|line| {
                aoc_process(line, &game_regex, &score_regex, &max_scores_map).unwrap_or_default()
            })
            .sum();

        assert_eq!(output, 8);
    }
}
