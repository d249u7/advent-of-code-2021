use std::fs;

const GRID_SIZE: usize = 5;

#[derive(Clone)]
enum Status {
    Marked,
    Unmarked,
}

#[derive(Clone)]
struct Cell {
    value: i64,
    status: Status,
}

#[derive(Clone)]
struct Line {
    cells: [Cell; GRID_SIZE],
}

#[derive(Clone)]
struct Matrix {
    rows: [Line; GRID_SIZE],
    columns: [Line; GRID_SIZE],
}

impl Cell {
    pub fn new(value: i64) -> Self {
        Self {
            value,
            status: Status::Unmarked,
        }
    }

    pub fn mark(&mut self) {
        self.status = Status::Marked;
    }
}

impl Line {
    pub fn new(cells: [Cell; GRID_SIZE]) -> Self {
        Self { cells }
    }

    pub fn count_marked(&self) -> u8 {
        self.cells
            .clone()
            .into_iter()
            .fold(0, |acc, elem| match elem.status {
                Status::Marked => acc + 1,
                _ => acc,
            })
    }
}

impl Matrix {
    pub fn new(rows: [Line; GRID_SIZE], columns: [Line; GRID_SIZE]) -> Self {
        Self { rows, columns }
    }

    pub fn check_winner(&self) -> bool {
        let mut is_winner = false;

        self.rows.clone().into_iter().for_each(|row| {
            match usize::try_from(row.count_marked()).unwrap() {
                GRID_SIZE => is_winner = true,
                _ => {}
            }
        });

        self.columns.clone().into_iter().for_each(|column| {
            match usize::try_from(column.count_marked()).unwrap() {
                GRID_SIZE => is_winner = true,
                _ => {}
            }
        });

        is_winner
    }
}

fn main() {
    let file =
        fs::read_to_string("/home/dragut/Projects/advent-of-code-2021/day4/inputs/example.txt")
            .expect("File could not be read");

    let lines: Vec<&str> = file.split("\n").collect();
    let input = lines[0];
}
