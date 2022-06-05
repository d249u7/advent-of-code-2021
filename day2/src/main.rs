use std::fs;

fn main() {
    let inputs = fs::read_to_string("/home/dragut/Projects/advent-of-code-day-2/inputs/day2.txt").expect("msg");
    let commands = inputs.split("\n").collect::<Vec<&str>>();
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    
    for command in commands {
        let command_pieces = command.split(" ").collect::<Vec<&str>>();
        let direction = command_pieces[0];
        let amount = command_pieces[1].parse::<i32>().unwrap();

        if direction == "forward" {
            horizontal += amount;
            depth += amount*aim;
        } else if direction == "down" {
            aim += amount;
        } else {
            aim -= amount;
        }
    }

    println!("{}", horizontal*depth)
}
