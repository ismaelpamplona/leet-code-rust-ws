use std::mem::swap;

struct Solution;
impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut right = 0;
        let mut left = 0;
        let mut result = vec![];
        let mut got_it = false;

        for (i, c) in word.chars().enumerate() {
            result.push(c);
            if c == ch && !got_it {
                got_it = true;
                right = i;
            }
        }

        while left < right {
            result.swap(left, right);
            left += 1;
            right -= 1;
        }
        // result[0..=right].reverse();
        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let s = String::from("abcdefd");
        let expected = String::from("dcbaefd");
        let result = Solution::reverse_prefix(s, 'd');
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let s = String::from("xyxzxe");
        let expected = String::from("zxyxxe");
        let result = Solution::reverse_prefix(s, 'z');
        assert_eq!(result, expected);
    }

    #[test]
    fn case_03() {
        let s = String::from("abcd");
        let expected = String::from("abcd");
        let result = Solution::reverse_prefix(s, 'z');
        assert_eq!(result, expected);
    }
}
