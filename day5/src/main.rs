use std::fs;

#[derive(Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub fn calculate_vent_coordinates(&self) {
        let vents: Vec<Point> = vec![];

        let horizontal = &self.end.x - &self.start.x;
        let vertical = &self.end.y - &self.start.y;
        let slope = vertical / horizontal;
    }
}

fn main() {
    let input =
        fs::read_to_string("/home/dragut/Projects/advent-of-code-2021/day5/inputs/example.txt")
            .expect("File could not be read!");
    let lines = input
        .split("\n")
        .filter(|x| x.len() > 0)
        .collect::<Vec<&str>>();
    println!("{:?}", lines);
    let coordinates = lines
        .into_iter()
        .map(|line| line.split("->").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    println!("{:?}", coordinates);
}
