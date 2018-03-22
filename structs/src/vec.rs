use std::vec::Vec;

#[derive(Debug)]
pub enum Error {
    SizeMismatch,
}

pub fn length(v: &Vec<f32>) -> f32 {
    v.iter().fold(0., |sum, &val| sum + val.powi(2)).sqrt()
}

pub fn sub(a: &Vec<f32>, b: &Vec<f32>) -> Result<Vec<f32>, Error> {
    if a.len() == b.len() {
        let mut result = Vec::with_capacity(a.len());
        for i in 0..a.len() { // Slow but just a proof of concept
            result.push(a[i] - b[i]);
        }
        Ok(result)
    } else {
        Err(Error::SizeMismatch)
    }
}

pub fn distance(a: &Vec<f32>, b: &Vec<f32>) -> Result<f32, Error> {
    match sub(a, b) {
        Err(e) => Err(e),
        Ok(result) => Ok(length(&result)),
    }
}
