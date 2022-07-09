use std::fs;

const GRID_SIZE: usize = 5;

#[derive(Clone, Debug)]
enum Status {
    Marked,
    Unmarked,
}

#[derive(Clone, Debug)]
struct Cell {
    value: i64,
    status: Status,
}

#[derive(Clone, Debug)]
struct Line {
    cells: Vec<Cell>,
}

#[derive(Clone, Debug)]
struct Matrix {
    rows: Vec<Line>,
    did_win: bool,
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
    pub fn new(values: Vec<&str>) -> Self {
        let cells = values
            .into_iter()
            .map(|val| {
                let cell_val = val.parse::<i64>().unwrap();
                let cell = Cell::new(cell_val);
                cell
            })
            .collect::<Vec<Cell>>();

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
    pub fn new(rows: Vec<Line>) -> Self {
        Self {
            rows,
            did_win: false,
        }
    }

    pub fn mark_cell(&mut self, val: &str) {
        for row in self.rows.iter_mut() {
            for cell in row.cells.iter_mut() {
                match cell.status {
                    Status::Unmarked => {
                        if cell.value == val.parse::<i64>().unwrap() {
                            //println!("Before: {:?}", self.clone());
                            cell.mark();
                            // println!("After: {:?}", self);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn check_winner(&mut self) -> bool {
        let mut is_winner = false;

        self.rows.clone().into_iter().for_each(|row| {
            match usize::try_from(row.count_marked()).unwrap() {
                GRID_SIZE => is_winner = true,
                _ => {}
            }
        });

        if is_winner {
            self.did_win = true;
            return is_winner;
        }

        for idx in 0..GRID_SIZE {
            let mut marked_count = 0;
            self.rows
                .clone()
                .into_iter()
                .for_each(|row| match row.cells[idx].status {
                    Status::Marked => marked_count += 1,
                    Status::Unmarked => {}
                });

            if marked_count == 5 {
                self.did_win = true;
                is_winner = true;
            }
        }

        is_winner
    }

    pub fn get_sum(&self) -> i64 {
        let mut res: i64 = 0;
        for row in self.rows.clone().into_iter() {
            for cell in row.cells.into_iter() {
                match cell.status {
                    Status::Unmarked => {
                        res += cell.value;
                    }
                    _ => {}
                }
            }
        }

        res
    }
}

fn main() {
    let file =
        fs::read_to_string("/home/dragut/Projects/advent-of-code-2021/day4/inputs/input.txt")
            .expect("File could not be read");

    let lines: Vec<&str> = file
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|y| y.trim())
        .collect();
    let inputs = lines[0]
        .split(",")
        .filter(|x| x.len() > 0)
        .collect::<Vec<&str>>();
    let matrix_data: Vec<Vec<&str>> = lines[1..]
        .to_vec()
        .into_iter()
        .map(|entries| entries.split(" ").filter(|elem| elem.len() > 0).collect())
        .collect();

    let mut matrices: Vec<Matrix> = vec![];

    for idx in (0..matrix_data.len()).step_by(5) {
        // let mut rows: Vec<Line> = vec![];
        // let mut columns: Vec<Line> = vec![];
        let rows = matrix_data[idx..idx + 5].to_vec().into_iter();
        let row_lines = rows.map(|row| Line::new(row)).collect::<Vec<Line>>();
        let matrix = Matrix::new(row_lines);
        matrices.push(matrix);
    }
    // println!("{:?}", matrices);
    let mut res;
    for input in inputs {
        for matrix_idx in 0..matrices.len() {
            matrices[matrix_idx].mark_cell(input);
            if matrices[matrix_idx].did_win {
                continue;
            }
            match matrices[matrix_idx].check_winner() {
                true => {
                    let sum = matrices[matrix_idx].get_sum();
                    res = sum * input.parse::<i64>().unwrap();
                    println!("{:?}", res);
                }
                false => {}
            }
        }
    }
}
