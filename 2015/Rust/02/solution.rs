fn part_one() {
    println!("Part 1");
    let presents = vec![
        "2x3x4",
        "1x1x10"
    ];

    let mut order_in_feet = 0;
    presents.iter().for_each(|present| {
        let dimensions: Vec<i32> = present
            .split('x')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let l = dimensions[0];
        let w = dimensions[1];
        let h = dimensions[2];

        order_in_feet += calc_wrapper_for_present(l, w ,h);
    });

    println!("Total Order: {}", order_in_feet);
}

fn calc_wrapper_for_present(l: i32, w: i32, h: i32) -> i32 {
    let lw = l * w;
    let wh = w * h;
    let hl = h * l;
    let &smallest_area = [lw, wh, hl].iter().min().unwrap();

    let area = 2 * lw + 2 * wh + 2 * hl + smallest_area;
    println!("Area: {}", area);

    return area
}

fn part_two() {
    println!("Part 2");
    let presents = vec![
        "2x3x4",
        "1x1x10"
    ];

    let mut order_in_feet = 0;
    presents.iter().for_each(|present| {
        let dimensions: Vec<i32> = present
            .split('x')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let l = dimensions[0];
        let w = dimensions[1];
        let h = dimensions[2];

        order_in_feet += calc_ribbon_for_present(l, w ,h);
    });

    println!("Total Order: {}", order_in_feet);
}

fn calc_ribbon_for_present(l: i32, w: i32, h: i32) -> i32 {
    let mut sorted = vec![l, w, h];
    sorted.sort();

    let perimeter_a = sorted[0] * 2;
    let perimeter_b = sorted[1] * 2;
    let volume = l * w * h;

    let ribbon_length = perimeter_a + perimeter_b + volume;
    println!("Ribbon length: {}", ribbon_length);

    return ribbon_length
}

fn main() {
    part_one();
    part_two();
}
