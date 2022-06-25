pub mod hello;

#[cfg(test)]
mod tests {
    const ENGLISH: &str = "English";
    const SPANISH: &str = "Spanish";
    const FRANCH: &str = "Franch";
    use crate::hello;
    #[test]
    fn test_hello() {
        let mut name: String = "Chris".to_string();
        let got = hello::hello(&mut name, ENGLISH);
        let want: String = String::from("Hello, Chris");
        assert_eq!(got, want);
    }

    #[test]
    fn test_hello_use_empty_string() {
        let mut name: String = "".to_string();
        let got = hello::hello(&mut name, ENGLISH);
        let want: String = String::from("Hello, World");
        assert_eq!(got, want);
    }

    #[test]
    fn test_hello_in_spanish() {
        let mut name: String = "Elodie".to_string();
        let got = hello::hello(&mut name, SPANISH);
        let want: String = String::from("Hola, Elodie");
        assert_eq!(got, want);
    }
    #[test]
    fn test_hello_in_franch() {
        let mut name: String = "Elodie".to_string();
        let got = hello::hello(&mut name, FRANCH);
        let want: String = String::from("Bonjour, Elodie");
        assert_eq!(got, want);
    }
}
