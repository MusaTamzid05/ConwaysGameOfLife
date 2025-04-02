use crate::game_cell::Cell;
use std::io::{self};

pub struct World {
    pub size: i32,
    pub cells: Vec<Vec<Cell>>,

}

impl World {
    pub fn new(size: i32) -> Self {
        let mut cells: Vec<Vec<Cell>> = Vec::new();

        for row in 0..size {
            let mut current_row_cells: Vec<Cell> = Vec::new();

            for col in 0..size {
                current_row_cells.push(Cell::new(row, col, false));
            }

            cells.push(current_row_cells);

        }

        /*
        cells[10][9].alive = true;
        cells[10][10].alive = true;
        cells[10][11].alive = true;
        */

        /*
        cells[10][5].alive = true;
        cells[10][6].alive = true;
        cells[9][6].alive = true;
        cells[8][5].alive = true;
        cells[8][6].alive = true;
        */

        cells[10][5].alive = true;
        cells[11][6].alive = true;
        cells[12][4].alive = true;
        cells[12][5].alive = true;
        cells[12][6].alive = true;

        Self { size, cells }
    }

    fn render(&self) {
        for row in 0..self.size {
            for col in 0..self.size {
                let cell: &Cell = &self.cells[row as usize][col as usize];
                cell.render();
            }
            println!("");

        }

    }


    pub fn update(&mut self) {
        let mut target_cell_data_list : Vec<(i32, i32, bool)> = Vec::new();

        for row in 0..self.size {
            for col in 0..self.size {
                let cell: &Cell = &self.cells[row as usize][col as usize];
                let alive_score: i32 = self.get_alive_score_of(cell);
                let alive: bool = cell.alive;

                if alive_score <= 1 && alive  {
                    // not enough neighbours, die !!
                    target_cell_data_list.push((row, col, false));
                }

                else if alive_score >= 2 && alive_score <= 3  && alive  {
                    // just enough neighbours, survice
                    target_cell_data_list.push((row, col, true));
                }

                else if alive_score > 3  && alive  {
                    // overpolulation, must die
                    target_cell_data_list.push((row, col, false));
                }

                else if alive_score ==  3 && alive == false {
                    // enough neighbours, time to be born !!
                    target_cell_data_list.push((row, col, true));
                }
            }
        }

        for (row, col, alive) in target_cell_data_list {
            self.cells[row as usize][col as usize].alive = alive;



        }



    }

    pub fn get_alive_score_of(&self, cell: &Cell) -> i32 {
        let mut alive_score: i32 = 0;
            let neighbour_info_list: Vec<(i32, i32)> = cell.get_neighbour_info_list();

            for (n_row, n_col) in neighbour_info_list {
                let mut valid: bool = false;

                if (n_row >= 0 && n_row < self.size) && (n_col >= 0 && n_col< self.size) {
                    valid = true;
                }

                if valid == false {
                    continue;
                }

                if self.cells[n_row as usize][n_col as usize].alive {
                    alive_score += 1;
                }
            }

        alive_score
    }

    pub fn start(&mut self) {
        loop {
            self.render();
            let mut input: String = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read enter");

            self.update();

        }
    }

}
