fn greet() {
    println!("Hello rust");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        greet();
    }
}
