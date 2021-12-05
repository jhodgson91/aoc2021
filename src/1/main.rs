use std::fs::read_to_string;

fn window_compare(windows: &&[&[i32]]) -> bool {
    let w1 = windows[0];
    let w2 = windows[1];

    w1.iter().sum::<i32>() < w2.iter().sum::<i32>()
}

fn main() {
    let input = read_to_string("src/1/input.txt").expect("failed to open input");
    let numbers: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();

    let result = numbers.windows(2).filter(|x| x[0] < x[1]).count();
    println!("Part 1: {} increases", result);

    let result = numbers
        .windows(3)
        .collect::<Vec<&[i32]>>()
        .windows(2)
        .filter(window_compare)
        .count();
    println!("Part 2: {} increases", result);
}
