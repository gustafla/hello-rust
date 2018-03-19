enum Language {
    English, German, Finnish,
}

fn greet(language: Language) {
    let greeting = match language {
        Language::English => "Hello",
        Language::German =>  "Hallo",
        Language::Finnish => "Hei",
    };
    println!("{} Rust", greeting);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        greet(Language::Finnish)
    }
}
