extern crate rand;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;
use sdl2::pixels;
use self::rand::{Rng, thread_rng};
use std::vec::Vec;

pub fn modulo(a: isize, b: isize) -> isize {
    ((a % b) + b) % b
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

struct State {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl State {
    fn new(w: usize, h: usize) -> State {
        State {
            width: w, height: h,
            cells: vec![vec![Cell::Dead; h]; w],
        }
    }

    fn sprinkle(&mut self) -> Vec<Point> {
        let mut rng = rand::thread_rng();
        let mut points = Vec::<Point>::with_capacity(self.width*self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                match rng.gen_range(0, 8) {
                    0 => {
                        self.set_cell(x, y);
                        points.push(Point::new(x as i32, y as i32));
                    }
                    _ => {}
                }
            }
        }
        points
    }

    fn set_cell(&mut self, x: usize, y: usize) -> Point {
        self.cells[x][y] = Cell::Alive;
        Point::new(x as i32, y as i32)
    }

    fn get_cell(&self, x: isize, y: isize) -> Cell {
        // Wrap around edges
        let x = modulo(x, self.width as isize);
        let y = modulo(y, self.height as isize);
        self.cells[x as usize][y as usize]
    }

    fn count_neighbors(&self, x: isize, y: isize) -> i32 {
        let mut n = 0i32;
        for i in x-1 .. x+2 {
            for j in y-1 .. y+2 {
                if i == x && j == y {continue;}
                match self.get_cell(i, j) {
                    Cell::Alive => n += 1,
                    _ => {},
                }
            }
        }
        n
    }
}

pub struct Game {
    state: State,
    points: Vec<Point>,
}

impl Game {
    pub fn new(w: usize, h: usize) -> Game {
        Game {
            state: State::new(w, h),
            points: Vec::<Point>::with_capacity(w*h),
        }
    }

    pub fn with_randomized(mut self) -> Game {
        self.points.extend(self.state.sprinkle());
        self
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
        canvas.draw_points(self.points.as_slice())
    }

    pub fn next_state(mut self) -> Game {
        // Store old state
        let old_state = self.state;
        // Make a clean state
        self.state = State::new(old_state.width, old_state.height);
        // Clear points
        self.points.clear();

        // Iterate positions
        for x in 0..self.state.width {
            for y in 0..self.state.height {
                let alive_neighbors = old_state.count_neighbors(x as isize, y as isize);
                match old_state.get_cell(x as isize, y as isize) {
                    Cell::Alive => {
                        match alive_neighbors {
                            2 ... 3 => self.points.push(self.state.set_cell(x, y)),
                            _ => {},
                        }
                    },
                    Cell::Dead => {
                        match alive_neighbors {
                            3 => self.points.push(self.state.set_cell(x, y)),
                            _ => {},
                        }
                    },
                }
            }
        }
        self
    }
}
