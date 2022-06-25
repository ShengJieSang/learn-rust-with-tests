const ENGLISH_HELLO_PREFIX: &str = "Hello, ";
const SPANISH_HELLO_PREFIX: &str = "Hola, ";
const FRANCH_HELLO_PREFIX: &str = "Bonjour, ";

pub fn hello(name: &mut String, language: &str) -> String {
    if *name == "".to_string() {
        *name = "World".to_string();
    }
    return format!("{}{}", greeting_prefix(language), name);
}

fn greeting_prefix(language: &str) -> &str {
    match language {
        "Spanish" => SPANISH_HELLO_PREFIX,
        "Franch" => FRANCH_HELLO_PREFIX,
        _ => ENGLISH_HELLO_PREFIX,
    }
}
