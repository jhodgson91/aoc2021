fn most_common(values: &[u16], column: usize) -> u16 {
    let num_ones = values.iter().filter(|x| (*x >> column & 1) == 1).count();
    (num_ones * 2 >= values.len()) as u16
}

fn least_common(values: &[u16], column: usize) -> u16 {
    let num_ones = values.iter().filter(|x| (*x >> column & 1) == 1).count();
    (num_ones * 2 < values.len()) as u16
}

fn power_consumption(values: &[u16]) -> u32 {
    let mut gamma = 0u32;
    for i in 0..12 {
        let x = most_common(values, i);
        if x > 0 {
            gamma |= 1 << i
        }
    }

    const MASK: u32 = 0b111111111111;
    let epsilon = MASK & !gamma;
    epsilon * gamma
}

fn life_support_rating(values: Vec<u16>) -> u32 {
    let mut oxy_values = values.clone();
    let mut co2_values = values;

    for column in (0..12).rev() {
        let oxy_cmp = most_common(&oxy_values, column);
        let co2_cmp = least_common(&co2_values, column);

        if oxy_values.len() > 1 {
            oxy_values.retain(|v| (v >> column & 1) == oxy_cmp);
        }

        if co2_values.len() > 1 {
            co2_values.retain(|v| (v >> column & 1) == co2_cmp);
        }
    }

    oxy_values[0] as u32 * co2_values[0] as u32
}

fn main() {
    let input: Vec<u16> = std::fs::read_to_string("src/3/input.txt")
        .expect("failed to read input")
        .lines()
        .map(|s| u16::from_str_radix(s, 2).expect("invalid input"))
        .collect();

    println!("Part 1: power consumption = {}", power_consumption(&input));
    println!(
        "Part 2: life support rating = {}",
        life_support_rating(input)
    );
}
