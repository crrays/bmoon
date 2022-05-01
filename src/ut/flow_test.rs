#[cfg(test)]
mod tests {
    #[test]
    fn basic_check1() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn basic_check2() {
        assert!(1 < 2);
    }

    #[test]
    fn basic_check3() {
        assert!(1 == 1);
    }
}