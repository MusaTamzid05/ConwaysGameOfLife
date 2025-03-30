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

    pub fn start(&self) {
        self.render();
    }

}
