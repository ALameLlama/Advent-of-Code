use regex::Regex;

pub fn run(only_part1: bool, only_part2: bool) {
    let input = include_str!("../../input/day03.txt");

    println!("Day 03");
    if !only_part2 {
        println!("Part 1 answer: {}", part1(input));
    }
    if !only_part1 {
        println!("Part 2 answer: {}", part2(input));
    }
}

fn part1(input: &str) -> i64 {
    let mul_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").expect("failed");
    let mul_values = Regex::new(r"mul\((\d+),(\d+)\)").expect("failed");

    let mut total = 0;

    for muls in mul_regex.captures_iter(input) {
        let mul = muls.get(0).unwrap().as_str();
        if let Some(values) = mul_values.captures(mul) {
            let first = values.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let second = values.get(2).unwrap().as_str().parse::<i64>().unwrap();

            total += first * second;
        };
    }

    total
}

fn part2(input: &str) -> i64 {
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").expect("failed");
    let regex_values = Regex::new(r"mul\((\d+),(\d+)\)").expect("failed");

    let mut total = 0;
    let mut enabled = true;

    for muls in mul_regex.captures_iter(input) {
        let matched = muls.get(0).unwrap().as_str();

        match matched {
            "do()" => {
                enabled = true;
                continue;
            }
            "don't()" => {
                enabled = false;
                continue;
            }
            _ => {}
        }

        if enabled && matched.starts_with("mul") {
            if let Some(values) = regex_values.captures(matched) {
                let first = values.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let second = values.get(2).unwrap().as_str().parse::<i64>().unwrap();

                total += first * second;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
