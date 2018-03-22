mod va; // "Include" src/va.rs

fn main() {
    let pi = 3.14159265_f32;

    // Define two points across an imaginary circle with r=2
    let point_a = vec![(pi/4f32).cos(), (pi/4f32).sin()];
    let point_b = vec![((pi/4f32)*5f32).cos(), ((pi/4f32)*5f32).sin()];
    println!("{:?} and {:?}", point_a, point_b);

    // Should print "distance 2" (r * 2)
    println!("distance {}", va::distance(&point_a, &point_b));

    // Test arithmetic
    println!("{:?}", va::add(&vec![0_f32, 0_f32], &vec![1_f32, -1_f32]));
    println!("{:?}", va::sub(&vec![0_f32, 0_f32], &vec![1_f32, -1_f32]));
    println!("{:?}", va::mul(&vec![1_f32, 1_f32], &vec![2_f32, -2_f32]));
    println!("{:?}", va::div(&vec![1_f32, 1_f32], &vec![2_f32, -2_f32]));
}
