mod va; // "Include" src/va.rs

use std::f32;

fn main() {
    let pi = f32::consts::PI;

    // Define two points across an imaginary circle with r=1
    let point_a = vec![(pi/4f32).cos(), (pi/4f32).sin()];
    let point_b = vec![((pi/4f32)*5f32).cos(), ((pi/4f32)*5f32).sin()];
    println!("a = {:?} and b = {:?}", point_a, point_b);

    // Should be -1
    println!("a dot b = {}", va::dot(&point_a, &point_b));

    // Should print "distance 2" (r * 2)
    println!("distance {}", va::distance(&point_a, &point_b));

    // Test arithmetic
    println!("{:?}", va::add(&vec![0_f32, 0_f32], &vec![1_f32, -1_f32]));
    println!("{:?}", va::sub(&vec![0_f32, 0_f32], &vec![1_f32, -1_f32]));
    println!("{:?}", va::mul(&vec![1_f32, 1_f32], &vec![2_f32, -2_f32]));
    println!("{:?}", va::div(&vec![1_f32, 1_f32], &vec![2_f32, -2_f32]));

    // Other ops
    println!("(1, 0, 0) cross (0, 1, 0) = {:?}",
        va::cross(&vec![1f32, 0f32, 0f32], &vec![0f32, 1f32, 0f32]));
    println!("normalize (2, 0, 2) = {:?}",
        va::normalize(&vec![2f32, 0f32, 2f32]));
    println!("length(normalize (2, 0, 2)) = {:?}",
        va::length(&va::normalize(&vec![2f32, 0f32, 2f32])));
}
