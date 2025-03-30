

pub struct World {
    pub size: i32
}

impl World {
    pub fn new(size: i32) -> Self {
        Self { size }
    }

    pub fn start(&self) {
        println!("Creating world of size {}", self.size);
    }
}
