#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn distance_from(&self, other: &Point) -> f32 {
        // Destructure to variables ax, ay, bx, by
        let Point {x: ax, y: ay} = *self;
        let Point {x: bx, y: by} = *other;
        let dx = ax - bx;
        let dy = ay - by;

        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        // Nested destructuring
        let Rectangle {a: Point {x: ax, y: ay}, 
            b: Point {x: bx, y: by}} = *self;
        let dx = bx - ax;
        let dy = by - ay;

        dx * dy
    }
}

fn main() {
    let point: Point = Point {x: 0.5, y: 0.2};
    println!("{:?}", point);
    println!("{}", point.distance_from(&Point {x: 0., y: 0.}));

    let copy = Point {..point};
}
