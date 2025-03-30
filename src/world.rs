use crate::game_cell::Cell;

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

        cells[10][10].alive = true;

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


    fn update(&mut self) {
        let mut alive_data_list: Vec<(i32, i32)> = Vec::new();

        for row in 0..self.size {
            for col in 0..self.size {
                let cell: &Cell = &self.cells[row as usize][col as usize];
                if cell.alive {
                    alive_data_list.push((row, col));
                }
            }
        }

        for (row, col) in alive_data_list {
            let cell: &Cell = &self.cells[row as usize][col as usize];
            let neighbour_info_list: Vec<(i32, i32)> = cell.get_neighbour_info_list();

            for (n_row, n_col) in neighbour_info_list {
                self.cells[n_row as usize][n_col as usize].alive = true;
            }


        }



    }

    pub fn start(&mut self) {
        self.update();
        self.render();
    }

}
