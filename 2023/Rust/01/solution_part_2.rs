use regex::Regex;

fn main() {
    let input = include_str!("./input2.txt");


    let dig = Regex::new(r"(?<num>\d|one|two|three|four|five|six|seven|eight|nine)").expect("Failed to compile regex");
    let dig_rev = Regex::new(r"(?<num>\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").expect("Failed to compile reverse regex");

    let output: i32 = input
        .lines()
        .map(|line| aoc_process(line, &dig, &dig_rev).unwrap_or_default())
        .sum();

    println!("{output}");
}

fn aoc_process(input: &str, dig: &Regex, dig_rev: &Regex) -> Result<i32, &'static str> {
    if input.is_empty() {
        return Ok(0);
    }

    let caps_first = dig.captures(input).ok_or("Failed to match first capture")?;

    let actual_dig_first = match caps_first["num"].as_ref() {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => &caps_first["num"],
    };
    
    let binding = input.chars().rev().collect::<String>();
    let caps_last = dig_rev.captures(binding.as_str()).ok_or("Failed to match last capture")?;

    let actual_dig_last = match caps_last["num"].as_ref() {
        "eno" => "1",
        "owt" => "2",
        "eerht" => "3",
        "ruof" => "4",
        "evif" => "5",
        "xis" => "6",
        "neves" => "7",
        "thgie" => "8",
        "enin" => "9",
        _ => &caps_last["num"],
    };

    let count: i32 = [actual_dig_first.to_string(), actual_dig_last.to_string()]
        .concat()
        .parse::<i32>()
        .map_err(|_| "Failed to parse into i32")?;
        
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

        let dig = Regex::new(r"(?<num>\d|one|two|three|four|five|six|seven|eight|nine)").expect("Failed to compile regex");
        let dig_rev = Regex::new(r"(?<num>\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").expect("Failed to compile reverse regex");

        let output: i32 = input
            .lines()
            .map(|line| aoc_process(line, &dig, &dig_rev).unwrap_or_default())
            .sum();

        assert_eq!(output, 281);
    }
}