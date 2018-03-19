use std::fmt;

enum Language {
    English, German, Finnish,
}

struct Greeter {
    language: Language
}

impl fmt::Display for Greeter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let greeting = match self.language {
            Language::English => "Hello",
            Language::German =>  "Hallo",
            Language::Finnish => "Hei",
        };
        write!(f, "{} Rust", greeting)
    }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("{}", Greeter::new().with_language(Language::Finnish));
    }
}
