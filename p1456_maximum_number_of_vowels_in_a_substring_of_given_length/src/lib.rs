struct Solution;
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let sv: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut curr = 0;
        let mut max = 0;

        for right in 0..sv.len() {
            if curr == k {
                return curr;
            }
            if Self::is_vowel(sv[right]) {
                curr += 1;
            }

            if right >= k as usize {
                if Self::is_vowel(sv[left]) {
                    curr -= 1;
                }
                left += 1;
            }

            max = std::cmp::max(max, curr);
        }

        max
    }

    fn is_vowel(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let s = String::from("abciiidef");
        let result = Solution::max_vowels(s, 3);
        assert_eq!(result, 3);
    }

    #[test]
    fn case_02() {
        let s = String::from("aeiou");
        let result = Solution::max_vowels(s, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn case_03() {
        let s = String::from("leetcode");
        let result = Solution::max_vowels(s, 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn case_04() {
        let s = String::from("ramadan");
        let result = Solution::max_vowels(s, 2);
        assert_eq!(result, 1);
    }

    #[test]
    fn case_05() {
        let s = String::from("zpuerktejfp");
        let result = Solution::max_vowels(s, 3);
        assert_eq!(result, 2);
    }
}
