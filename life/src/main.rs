extern crate sdl2;
extern crate rand;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels;
use std::vec::Vec;
use rand::{Rng, thread_rng};

const WIDTH: i32 = 400;
const HEIGHT: i32 = 400;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

struct Game {
    state: [[Cell; HEIGHT as usize]; WIDTH as usize],
}

impl Game {
    fn new() -> Game {
        Game {state: [[Cell::Dead; HEIGHT as usize]; WIDTH as usize]}
    }

    fn with_randomized() -> Game {
        let mut game = Game::new();
        let mut rng = rand::thread_rng();
        for x in 0..game.state.len() {
            for y in 0..game.state[x].len() {
                if rng.gen() {
                    game.state[x][y] = Cell::Alive;
                }
            }
        }
        game
    }

    fn get_cell(&self, x: isize, y: isize) -> Cell {
        // Wrap around edges
        let x = modulo(x, WIDTH as isize);
        let y = modulo(y, HEIGHT as isize);
        self.state[x as usize][y as usize]
    }

    fn count_alive(&self, x: isize, y: isize) -> i32 {
        let mut n = 0i32;
        for i in x-1 .. x+2 {
            for j in y-1 .. y+2 {
                if i == x && j == y {continue;}
                match self.get_cell(x, y) {
                    Cell::Alive => {n += 1;},
                    _ => {},
                }
            }
        }
        n
    }

    fn next_state(self) -> Game {
        let mut next = Game::new();
        for x in 0..self.state.len() {
            for y in 0..self.state[x].len() {
                let alive_neighbors = self.count_alive(x as isize, y as isize);
                match self.state[x][y] {
                    Cell::Alive => {
                        match alive_neighbors {
                            2 ... 3 => {next.state[x][y] = Cell::Alive;},
                            _ => {},
                        }
                    },
                    Cell::Dead => {
                        match alive_neighbors {
                            3 => {next.state[x][y] = Cell::Alive;},
                            _ => {},
                        }
                    },
                }
            }
        }
        next
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let mut points = Vec::<sdl2::rect::Point>::with_capacity(((WIDTH*HEIGHT)/2) as usize);
        for x in 0..self.state.len() {
            for y in 0..self.state[x].len() {
                if self.state[x][y] == Cell::Alive {
                    points.push(sdl2::rect::Point::new(x as i32, y as i32));
                }
            }
        }
        canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
        canvas.draw_points(points.as_slice())
    }
}

fn modulo(a: isize, b: isize) -> isize {
    ((a % b) + b) % b
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video.window("GOL", WIDTH as u32, HEIGHT as u32)
        .position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = sdl_context.event_pump().unwrap();

    let mut game = Game::with_randomized();

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
        game = game.next_state();
    }
}
