enum Language {
    English, German,
}

fn greet(language: Language) {
    match language {
        Language::English => println!("Hello rust"),
        Language::German => println!("Hallo rust"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        greet(Language::German);
    }
}
