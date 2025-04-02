mod world;
mod game_cell;
mod window;

use window::MWindow;

fn main() {
    let mut window: MWindow = MWindow::new();
    window.start();
}
