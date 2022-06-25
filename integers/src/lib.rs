#[cfg(test)]
mod tests {
    use crate::integers;

    #[test]
    fn test_adder() {
        let sum = integers::add(2, 2);
        let expected = 4;
        assert_eq!(sum, expected);
    }
}

mod integers {
    pub fn add(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}
