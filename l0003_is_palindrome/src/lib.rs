struct Solution;
impl Solution {
    pub fn is_palindrome(word: String) -> bool {
        let mut left = 0 as usize;
        let mut right = word.len() - 1;

        while left < right {
            if Some(word.chars().nth(left)) != Some(word.chars().nth(right)) {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let result = Solution::is_palindrome(String::from("racecar"));
        assert_eq!(result, true);
    }

    #[test]
    fn case_02() {
        let result = Solution::is_palindrome(String::from("ismael"));
        assert_eq!(result, false);
    }

    #[test]
    fn case_03() {
        let result = Solution::is_palindrome(String::from("nun"));
        assert_eq!(result, true);
    }

    #[test]
    fn case_04() {
        let result = Solution::is_palindrome(String::from("mom"));
        assert_eq!(result, true);
    }

    #[test]
    fn case_05() {
        let result = Solution::is_palindrome(String::from("aibohphobia"));
        assert_eq!(result, true);
    }

    #[test]
    fn case_06() {
        let result = Solution::is_palindrome(String::from("rotator"));
        assert_eq!(result, true);
    }

    #[test]
    fn case_07() {
        let result = Solution::is_palindrome(String::from("aaaaa"));
        assert_eq!(result, true);
    }
}
