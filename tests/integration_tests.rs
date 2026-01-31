use rustcourse::myutils;

#[cfg(test)]
mod tests {
    use super::myutils::testing;

    #[test]
    fn test_add() {
        let result = testing::add(3, 5);
        assert_eq!(result, 8);
    }
}
