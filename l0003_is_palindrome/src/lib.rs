struct Solution;
impl Solution {
    pub fn is_palindrome(word: &str) -> bool {
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
        let result = Solution::is_palindrome("racecar");
        assert_eq!(result, true);
    }

    #[test]
    fn case_02() {
        let result = Solution::is_palindrome("ismael");
        assert_eq!(result, false);
    }

    #[test]
    fn case_03() {
        let result = Solution::is_palindrome("nun");
        assert_eq!(result, true);
    }

    #[test]
    fn case_04() {
        let result = Solution::is_palindrome("mom");
        assert_eq!(result, true);
    }

    #[test]
    fn case_05() {
        let result = Solution::is_palindrome("aibohphobia");
        assert_eq!(result, true);
    }

    #[test]
    fn case_06() {
        let result = Solution::is_palindrome("rotator");
        assert_eq!(result, true);
    }

    #[test]
    fn case_07() {
        let result = Solution::is_palindrome("aaaaa");
        assert_eq!(result, true);
    }

    #[test]
    fn case_08() {
        let result = Solution::is_palindrome("aabbaa");
        assert_eq!(result, true);
    }

    #[test]
    fn case_09() {
        let result = Solution::is_palindrome("flamengo");
        assert_eq!(result, false);
    }
}
