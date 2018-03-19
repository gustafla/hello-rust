enum Language {
    English, German, Finnish,
}

struct Greeter {
    language: Language
}

impl Greeter {
    fn new() -> Greeter {
        Greeter {
            language: Language::English,
        }
    }

    fn with_language(mut self, language: Language) -> Greeter {
        self.language = language;
        self
    }

    fn greet(self) {
        let greeting = match self.language {
            Language::English => "Hello",
            Language::German =>  "Hallo",
            Language::Finnish => "Hei",
        };
        println!("{} Rust", greeting);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        Greeter::new().with_language(Language::Finnish).greet()
    }
}
