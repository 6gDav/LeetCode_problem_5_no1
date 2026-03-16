pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        //entry
        if s == s.chars().rev().collect::<String>() {
            return s.to_string();
        }

        let mut dummy_string = String::new();

        //forward
        for ch in s.chars() {
            dummy_string.push(ch);

            if dummy_string.len() > 1 {
                let rev_dummy_string: String = dummy_string.chars().rev().collect();

                if rev_dummy_string == dummy_string {
                    return dummy_string.to_string();
                }
            }
        }

        dummy_string.clear();

        //backward
        for ch in s.chars().rev() {
            dummy_string.push(ch);

            if dummy_string.len() > 1 {
                let rev_dummy_string: String = dummy_string.chars().rev().collect();

                if rev_dummy_string == dummy_string {
                    return dummy_string.to_string();
                }
            }
        }

        dummy_string.clear();

        //from towo sides
        let mut chars = s.chars();
        for _ in 0..=s.len() {
            chars.next();
            chars.next_back();

            if chars.clone().eq(chars.clone().rev()) {
                return chars.clone().collect::<String>();
            }
        }

        "".to_string()
    }
}
