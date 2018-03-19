enum Language {
    English
}

fn greet(language: Language) {
    match language {
        Language::English => println!("Hello rust"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        greet(Language::English);
    }
}
