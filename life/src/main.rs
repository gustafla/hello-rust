extern crate sdl2;

mod game;

use game::Game;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video.window("GOL", WIDTH, HEIGHT)
        .position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = sdl_context.event_pump().unwrap();

    let mut game = Game::new(WIDTH as usize, HEIGHT as usize).with_randomized();

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                // KeyDown has field keycode, Some(keycode) expression shadows?
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Escape | Keycode::Q => break 'main,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        game.render(&mut canvas).unwrap();
        canvas.present();
        game = game.next();
    }
}
