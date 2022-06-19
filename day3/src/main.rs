use std::fs;

fn count_digits(binary_list: Vec<&str>, digit_index: usize) -> [i32; 2] {
    let mut one_freq = 0;
    let mut zero_freq = 0;

    for binary in binary_list {
        let digits: Vec<i32> = binary
            .split("")
            .filter(|d| d.len() == 1)
            .map(|d| d.parse::<i32>().unwrap())
            .collect();

        match digits[digit_index] {
            1 => one_freq += 1,
            _ => zero_freq += 1,
        }
    }

    [zero_freq, one_freq]
}

fn binary_to_decimal(binary: &str) -> i64 {
    let mut decimal: i64 = 0;
    const BASE: i64 = 2;
    let numbers = binary
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    for idx in 0..binary.len() {
        match numbers[idx] {
            1 => decimal += BASE.pow((binary.len() - 1 - idx).try_into().unwrap()),
            _ => {}
        }
    }

    decimal
}

fn main() {
    let inputs =
        fs::read_to_string("/home/dragut/Projects/advent-of-code-2021/day3/inputs/input.txt")
            .expect("msg");
    let binaries: Vec<&str> = inputs.split("\n").collect();
    let mut oxygen_generator_rating_binaries = binaries.clone();
    let mut co2_scrubber_rating_binaries = binaries.clone();
    let oxygen_generator_rating: i64;
    let co2_scrubber_rating: i64;
    let mut idx: usize = 0;
    while oxygen_generator_rating_binaries.len() > 1 || co2_scrubber_rating_binaries.len() > 1 {
        if oxygen_generator_rating_binaries.len() > 1 {
            let oxygen_freqs = count_digits(oxygen_generator_rating_binaries.clone(), idx);
            let oxygen_most_common;
            if &oxygen_freqs[0] > &oxygen_freqs[1] {
                oxygen_most_common = 0;
            } else {
                oxygen_most_common = 1;
            }

            oxygen_generator_rating_binaries = oxygen_generator_rating_binaries
                .into_iter()
                .filter(|x| {
                    x.chars().map(|x| x.to_string()).collect::<Vec<String>>()[idx]
                        .parse::<i128>()
                        .unwrap()
                        == oxygen_most_common
                })
                .collect::<Vec<&str>>();
        }

        if co2_scrubber_rating_binaries.len() > 1 {
            let co2_freqs = count_digits(oxygen_generator_rating_binaries.clone(), idx);
            let co2_most_common;
            if &co2_freqs[0] > &co2_freqs[1] {
                co2_most_common = 0;
            } else {
                co2_most_common = 1;
            }

            co2_scrubber_rating_binaries = co2_scrubber_rating_binaries
                .into_iter()
                .filter(|x| {
                    x.chars().map(|x| x.to_string()).collect::<Vec<String>>()[idx]
                        .parse::<i128>()
                        .unwrap()
                        != co2_most_common
                })
                .collect::<Vec<&str>>();
        }

        idx += 1;
    }

    co2_scrubber_rating = binary_to_decimal(&co2_scrubber_rating_binaries[0]);
    oxygen_generator_rating = binary_to_decimal(&oxygen_generator_rating_binaries[0]);

    let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;

    println!("{}", life_support_rating);
}
