struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let sv: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
            .collect();

        if sv.len() == 0 {
            return true;
        }
        let len = sv.len();
        let mut left = 0 as usize;
        let mut right = len - 1;

        while left < right {
            if sv[left] != sv[right] {
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
        let s = String::from("A man, a plan, a canal: Panama");
        let result = Solution::is_palindrome(s);
        assert_eq!(result, true);
    }

    #[test]
    fn case_02() {
        let s = String::from("race a car");
        let result = Solution::is_palindrome(s);
        assert_eq!(result, false);
    }

    #[test]
    fn case_03() {
        let s = String::from(" ");
        let result = Solution::is_palindrome(s);
        assert_eq!(result, true);
    }

    #[test]
    fn case_04() {
        let s = String::from("0P");
        let result = Solution::is_palindrome(s);
        assert_eq!(result, false);
    }
}
