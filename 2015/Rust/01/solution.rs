fn part_one() {
    println!("Part 1");
    let directions = vec![
        "(())",
        "()()",
        "(((",
        "(()(()(",
        "))(((((",
        "())",
        "))(",
        ")))",
        ")())())",
    ];

    // both directions needed to have & since they share the same value,
    // adding & shows borrowing a reference instead of taking ownership so we can read it without modifying it

    println!("Method 1");
    for direction in &directions {
        which_floor(direction);
    }

    // or

    println!("Method 2");
    directions.iter().for_each(|&directions| {
        calculate_floor(directions);
    })
}

fn which_floor(direction: &str) {
    let up: i32 = direction.matches("(").count() as i32;
    let down: i32 = direction.matches(")").count() as i32;
    println!("Floor: {}", up - down);
}

// Uses Fold and Match, Defaults values to 0,
// loops over every char and then uses match to increment the accumulator
fn calculate_floor(direction: &str) {
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
    // https://doc.rust-lang.org/std/keyword.match.html
    let (up, down) = direction.chars().fold((0, 0), |(up, down), char| {
        match char {
            '(' => (up + 1, down),
            ')' => (up, down + 1),
            _ => (up, down),
        }
    });

    println!("Floor: {}", up - down);
}

fn part_two() {
    println!("Part 2");
    let directions = vec![
        ")",
        "()())",
    ];

    // both directions needed to have & since they share the same value,
    // adding & shows borrowing a reference instead of taking ownership so we can read it without modifying it

    println!("Method 1");
    for direction in &directions {
        let index = index_of_first_basement_entry(direction);
        println!("index: {:?}", index);
    }

    // or

    println!("Method 2");
    directions.iter().for_each(|&directions| {
        let index = index_of_first_basement_entry_two(directions);
        println!("index: {:?}", index);
    })
}

fn index_of_first_basement_entry(direction: &str) -> i32 {
    let mut current_floor = 0;
    let mut current_instruction = 0;

    for instruction in direction.chars() {
        current_instruction += 1;

        if instruction == '(' {
            current_floor += 1
        }

        if instruction == ')' {
            current_floor -= 1;
        }

        if current_floor == -1 {
            return current_instruction;
        }
    }

    return 0;
}

// use enumerate to get index and value https://doc.rust-lang.org/stable/std/iter/struct.Enumerate.html
// use match instead of if checks
fn index_of_first_basement_entry_two(direction: &str) -> i32 {
    let mut current_floor = 0;

    for (index, instruction) in direction.chars().enumerate() {
        match instruction {
            '(' =>  current_floor += 1,
            ')' => current_floor -= 1,
            _ => {},
        }

        if current_floor == -1 {
            return (index + 1) as i32;
        }
    }

    return 0;
}

fn main() {
    part_one();
    part_two();
}
