extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video.window("GOL", 400, 400).position_centered()
        .opengl().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    
    let mut events = sdl_context.event_pump().unwrap();

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Escape | Keycode::Q => break 'main,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        canvas.present();
    }
}
