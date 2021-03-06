#[cfg(test)]
mod tests {
    #[test]
    fn test_repeat() {
        let repeated = repeat("a".to_string());
        let expected = "aaaaa".to_string();
        assert_eq!(repeated, expected);
    }

    // for 循环
    fn repeat(s: String) -> String {
        let mut s1 = "".to_string();
        for _ in 0..5 {
            s1 = format!("{}{}", s1, s);
        }
        s1
    }

    #[test]
    fn test_repeat_times() {
        let repeated = repeat_times("a".to_string(), 8);
        let expected = "aaaaaaaa".to_string();
        assert_eq!(repeated, expected);
    }

    // for 循环
    fn repeat_times(s: String, times: i32) -> String {
        let mut s1 = "".to_string();
        for _ in 0..times {
            s1 = format!("{}{}", s1, s);
        }
        s1
    }

    #[test]
    fn test_repeat_while_times() {
        let repeated = repeat_while_times("a".to_string(), 9);
        let expected = "aaaaaaaaa".to_string();
        assert_eq!(repeated, expected);
    }

    // while 循环
    fn repeat_while_times(s: String, times: i32) -> String {
        let mut i = 0;
        let mut s1 = "".to_string();
        while i < times {
            s1 = format!("{}{}", s1, s);
            i += 1;
        }
        s1
    }

    #[test]
    fn test_repeat_while() {
        let repeated = repeat_while("a".to_string());
        let expected = "aaaaa".to_string();
        assert_eq!(repeated, expected);
    }

    // while 循环
    fn repeat_while(s: String) -> String {
        let mut i = 0;
        let mut s1 = "".to_string();
        while i < 5 {
            s1 = format!("{}{}", s1, s);
            i += 1;
        }
        s1
    }
}
