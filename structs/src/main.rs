mod vec; // "Include" src/vec.rs

//#[derive(Debug)]
//struct Rectangle {
//    a: Vec2,
//    b: Vec2,
//}
//
//impl Rectangle {
//    fn area(self) -> f32 {
//        // destructuring
//        let d: Vec2 = self.b - self.a;
//        d.x * d.y
//    }
//}

fn main() {
    let point = vec![0.1_f32, 0.5_f32];
    println!("{:?}", point);
    println!("{}", vec::distance(&point, &vec![0.6_f32, 1.3_f32]).unwrap());
}
