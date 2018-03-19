use std::fmt;

#[derive(Debug)]
enum Language {
    English, German, Finnish,
}

#[derive(Debug)]
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

    // moves ownership of self to parameter self, because it's not a referece
    fn with_language(mut self, language: Language) -> Greeter {
        self.language = language;
        self // returns ownership of self (Greeter object) to caller
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let greeter = Greeter::new().with_language(Language::Finnish);
        assert_eq!(greeter.to_string(), "Hei Rust");
        println!("{:?}", greeter);
    }
}
