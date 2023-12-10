
#[cfg(test)]
mod tests {
    #[test]
    fn one_is_number() {
        assert!(is_number('1'));
    }
    #[test]
    fn all_numbers() {
        assert!(is_number('1'));
        assert!(is_number('2'));
        assert!(is_number('3'));
        assert!(is_number('4'));
        assert!(is_number('5'));
        assert!(is_number('6'));
        assert!(is_number('7'));
        assert!(is_number('8'));
        assert!(is_number('9'));

    }
}
