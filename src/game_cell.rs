

pub struct Cell {
    pub row: i32,
    pub col: i32,
    pub alive: bool,
}

impl Cell {
    pub fn new(row: i32, col: i32, alive: bool) -> Self {
        Self { row, col, alive }
    }

    pub fn render(&self) {
        if self.alive {
            print!("*");
        } else {

            print!(".");
        }

    }

}

