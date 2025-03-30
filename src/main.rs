mod world;
mod game_cell;

use world::World;

fn main() {
    let mut world: World = World::new(20);
    world.start();
}
