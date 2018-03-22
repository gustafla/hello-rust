mod vec; // "Include" src/vec.rs
use vec::Vec2;

#[derive(Debug)]
struct Rectangle {
    a: Vec2,
    b: Vec2,
}

impl Rectangle {
    fn area(self) -> f32 {
        // destructuring
        let d: Vec2 = self.b - self.a;
        d.x * d.y
    }
}

fn main() {
    let point: Vec2 = Vec2 {x: 0.5, y: 0.2};
    println!("{:?}", point);
    println!("{}", point.distance_from(&Vec2 {x: 0., y: 0.}));

    let copy = Vec2 {..point};
}
