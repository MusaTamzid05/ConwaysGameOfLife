mod world;
mod game_cell;

use world::World;

fn main() {
    let world: World = World::new(20);
    world.start();
}
