

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

    pub fn get_neighbour_info_list(&self) -> Vec<(i32, i32)> {
        let mut neighbour_info_list: Vec<(i32, i32)> = Vec::new();

        // above row 
        neighbour_info_list.push((self.row-1, self.col-1));
        neighbour_info_list.push((self.row-1, self.col));
        neighbour_info_list.push((self.row-1, self.col+1));

        // center row
        neighbour_info_list.push((self.row, self.col-1));
        neighbour_info_list.push((self.row, self.col+1));


        // below row
        neighbour_info_list.push((self.row+1, self.col-1));
        neighbour_info_list.push((self.row+1, self.col));
        neighbour_info_list.push((self.row+1, self.col+1));

        neighbour_info_list

    }

}

