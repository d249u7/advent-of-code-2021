use std::fs;

fn main() {
    let contents = fs::read_to_string("/home/dragut/Projects/advent-of-code-day-1/inputs/day1.txt").expect("msg");

    let contents_list: Vec<i32> = contents.split("\n").map(|x| x.parse::<i32>().unwrap()).collect();

    let mut result = 0;
    let mut prev_sum: i32 = contents_list[0] + contents_list[1] + contents_list[2]; 
    
    for idx in 1..contents_list.len()-2 {
        let sum = contents_list[idx] + contents_list[idx+1] + contents_list[idx+2];
        if sum > prev_sum {
            result += 1;
        }

        prev_sum = sum;
    }

    println!("{}", result)
}
