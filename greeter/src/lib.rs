enum Language {
    English, German, Finnish
}

fn greet(language: Language) {
    match language {
        Language::English => println!("Hello rust"),
        Language::German => println!("Hallo rust"),
        Language::Finnish => println!("Hei rust"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        greet(Language::Finnish);
    }
}
