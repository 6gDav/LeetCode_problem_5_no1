#[cfg(test)]
mod tests {
    use crate::solution::Solution as sol;

    #[test]
    fn longest_pal_substr_exp1() {
        assert_eq!(sol::longest_palindrome("babad".to_string()), "bab".to_string());
    }

    #[test]
    fn longest_pal_substr_exp2() {
        assert_eq!(sol::longest_palindrome("cbbd".to_string()), "bb".to_string());
    }
}