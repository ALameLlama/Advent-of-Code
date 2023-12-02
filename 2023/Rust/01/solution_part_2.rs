use regex::Regex;

const DIGIT_STRINGS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = include_str!("./input2.txt");

    let dig = Regex::new(r"(?<num>\d|one|two|three|four|five|six|seven|eight|nine)")
        .expect("Failed to compile regex");
    let dig_rev = Regex::new(r"(?<num>\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)")
        .expect("Failed to compile reverse regex");

    let output: u32 = input
        .lines()
        .map(|line| aoc_process(line, &dig, &dig_rev).unwrap_or_default())
        .sum();

    println!("{output}");
}

fn aoc_process(input: &str, dig: &Regex, dig_rev: &Regex) -> Result<u32, &'static str> {
    let caps_first = dig.captures(input).ok_or("Failed to match first capture")?;

    let actual_dig_first: u32 = match DIGIT_STRINGS.iter().position(|&s| s == &caps_first["num"]) {
        Some(index) => (index + 1) as u32,
        None => caps_first["num"]
            .parse()
            .map_err(|_| "Failed to parse cap")?,
    };

    let binding = input.chars().rev().collect::<String>();
    let caps_last = dig_rev
        .captures(binding.as_str())
        .ok_or("Failed to match last capture")?;

    let actual_dig_last: u32 = match DIGIT_STRINGS
        .iter()
        .position(|&s| s == &caps_last["num"].chars().rev().collect::<String>())
    {
        Some(index) => (index + 1) as u32,
        None => caps_last["num"]
            .parse()
            .map_err(|_| "Failed to parse cap")?,
    };

    let count: u32 = [actual_dig_first.to_string(), actual_dig_last.to_string()]
        .concat()
        .parse::<u32>()
        .map_err(|_| "Failed to parse into u32")?;

    println!("{}", count);
    return Ok(count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

        let dig = Regex::new(r"(?<num>\d|one|two|three|four|five|six|seven|eight|nine)")
            .expect("Failed to compile regex");
        let dig_rev = Regex::new(r"(?<num>\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)")
            .expect("Failed to compile reverse regex");

        let output: u32 = input
            .lines()
            .map(|line| aoc_process(line, &dig, &dig_rev).unwrap_or_default())
            .sum();

        assert_eq!(output, 281);
    }
}
