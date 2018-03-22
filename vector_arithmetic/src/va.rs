use std::vec::Vec;
use std::cmp;

fn operate<F: Fn(&f32, &f32) -> f32>(a: &[f32], b: &[f32], op: F) -> Vec<f32> {
    // <https://users.rust-lang.org/t/how-to-zip-two-slices-efficiently/2048>

    // Allocate a result
    let len = cmp::min(a.len(), b.len());
    let mut result = Vec::with_capacity(len);

    // Limit slices by shadowing their variables for optimizer
    let a = &a[..len];
    let b = &b[..len];

    for i in 0..len {
        result.push(op(&a[i], &b[i]));
    }

    result
}

pub fn add(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
    operate(a.as_slice(), b.as_slice(), |a, b| a + b)
}

pub fn sub(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
    operate(a.as_slice(), b.as_slice(), |a, b| a - b)
}

pub fn mul(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
    operate(a.as_slice(), b.as_slice(), |a, b| a * b)
}

pub fn div(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
    operate(a.as_slice(), b.as_slice(), |a, b| a / b)
}

pub fn dot(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    mul(a, b).iter().sum()
}

pub fn length(v: &Vec<f32>) -> f32 {
    v.iter().fold(0., |sum, &val| sum + val.powi(2)).sqrt()
}

pub fn distance(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    length(&sub(a, b))
}
