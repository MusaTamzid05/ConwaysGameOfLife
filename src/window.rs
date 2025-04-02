use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use std::time::Duration;


const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const CELL_COUNT: i32 = 20;


pub struct MWindow {
    context: Sdl,
    canvas: Canvas<Window>,
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

        Self {
            context,
            canvas
        }



    }

    fn render(&mut self) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();



    }

    pub fn start(&mut self) {
        let mut event_pump = self.context.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => break 'running,
                    _=> {}
                }

            }

            self.render();
            std::thread::sleep(Duration::from_millis(16));

        }
    }
}


