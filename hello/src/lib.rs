pub mod hello;

#[cfg(test)]
mod tests {
    use crate::hello;
    #[test]
    fn test_hello() {
        let name: String = "Chris".to_string();
        let got = hello::hello(&name);
        let want: String = String::from("Hello, Chris");
        assert_eq!(got, want);
    }
}
