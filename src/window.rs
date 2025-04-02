use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use crate::world::World;
use crate::game_cell::Cell;



const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const CELL_COUNT: i32 = 20;


pub struct MWindow {
    context: Sdl,
    canvas: Canvas<Window>,
    world: World,
    rect_size: u32,
}

impl MWindow {
    pub fn new() -> Self {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();

        let window = video_subsystem
            .window("Window", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let world: World = World::new(CELL_COUNT);
        let rect_size: u32 = WINDOW_WIDTH / u32::try_from(CELL_COUNT).unwrap();

        Self {
            context,
            canvas,
            world,
            rect_size,
        }



    }

    fn render(&mut self) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();

        let cells: &Vec<Vec<Cell>> = &self.world.cells;
        let size: i32 = i32::try_from(self.rect_size).unwrap();

        for cell_row in cells {
            for cell in cell_row {
                let pos_y: i32 = cell.row * size;
                let pos_x: i32 = cell.col * size;
                let rect: Rect = Rect::new(pos_x, pos_y, self.rect_size, self.rect_size);

                if cell.alive {
                    self.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
                    self.canvas.fill_rect(rect).unwrap();
                }

                self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
                self.canvas.draw_rect(rect).unwrap();


            }
        }

        self.canvas.present();



    }

    pub fn start(&mut self) {
        let mut event_pump = self.context.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => break 'running,
                    Event::KeyDown { keycode: Some(Keycode::Return), ..} => {
                        self.world.update();
                    }
                    _=> {}
                }

            }

            self.render();
            std::thread::sleep(Duration::from_millis(16));

        }
    }
}


