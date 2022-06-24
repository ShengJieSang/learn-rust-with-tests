const ENGLISH_HELLO_PREFIX: &str = "Hello, ";

pub fn hello(name: &String) -> String {
    return format!("{}{}", ENGLISH_HELLO_PREFIX, name);
}
