use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_vec: Vec<char> = s.chars().collect();
        let t_vec: Vec<char> = t.chars().collect();
        let (mut i, mut j) = (0, 0);

        while i < s_vec.len() && j < t_vec.len() {
            if s_vec[i] == t_vec[j] {
                i += 1;
            }
            j += 1;
        }

        return i == s_vec.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        let result_1 = Solution::is_subsequence(s, t);
        assert_eq!(result_1, true);
    }

    #[test]
    fn case_02() {
        let s = String::from("axc");
        let t = String::from("ahbgdc");
        let result_1 = Solution::is_subsequence(s, t);
        assert_eq!(result_1, false);
    }

    #[test]
    fn case_03() {
        let s = String::from("");
        let t = String::from("ahbgdc");
        let result_1 = Solution::is_subsequence(s, t);
        assert_eq!(result_1, true);
    }

    #[test]
    fn case_04() {
        let s = String::from("acb");
        let t = String::from("ahbgdc");
        let result_1 = Solution::is_subsequence(s, t);
        assert_eq!(result_1, false);
    }
}
