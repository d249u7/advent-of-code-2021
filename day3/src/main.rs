use std::fs;

fn main() {
    let inputs =
        fs::read_to_string("/home/dragut/Projects/advent-of-code-2021/day3/inputs/input.txt")
            .expect("msg");
    let binaries: Vec<&str> = inputs.split("\n").collect();
    let binary_length = binaries[0].len();
    let mut one_freqs: Vec<i32> = vec![0; binary_length];
    let mut zero_freqs: Vec<i32> = vec![0; binary_length];

    for binary in binaries {
        let digits: Vec<i32> = binary
            .split("")
            .filter(|d| d.len() == 1)
            .map(|d| d.parse::<i32>().unwrap())
            .collect();

        for digit_index in 0..digits.len() {
            match digits[digit_index] {
                1 => one_freqs[digit_index] += 1,
                _ => zero_freqs[digit_index] += 1,
            }
        }
    }

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for idx in 0..binary_length {
        let base: i32 = 2;
        if one_freqs[idx] >= zero_freqs[idx] {
            gamma_rate += base.pow((binary_length - 1 - idx).try_into().unwrap());
        } else {
            epsilon_rate += base.pow((binary_length - 1 - idx).try_into().unwrap());
        }
    }

    println!(
        "{:?} {:?} {}",
        one_freqs,
        zero_freqs,
        gamma_rate * epsilon_rate
    )
}
