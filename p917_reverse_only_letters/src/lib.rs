struct Solution;
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut str_vec: Vec<char> = s.chars().collect();
        let mut left: usize = 0;
        let mut right = s.len() - 1;

        while left < right {
            if !str_vec[left].is_ascii_alphabetic() {
                left += 1;
            }

            if !str_vec[right].is_ascii_alphabetic() {
                right -= 1;
            }

            if str_vec[left].is_ascii_alphabetic() && str_vec[right].is_ascii_alphabetic() {
                let tmp = str_vec[left];
                str_vec[left] = str_vec[right];
                str_vec[right] = tmp;
                left += 1;
                right -= 1;
            }
        }
        str_vec.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let s = String::from("ab-cd");
        let expected = String::from("dc-ba");
        let result = Solution::reverse_only_letters(s);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let s = String::from("a-bC-dEf-ghIj");
        let expected = String::from("j-Ih-gfE-dCba");
        let result = Solution::reverse_only_letters(s);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_03() {
        let s = String::from("Test1ng-Leet=code-Q!");
        let expected = String::from("Qedo1ct-eeLg=ntse-T!");
        let result = Solution::reverse_only_letters(s);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_04() {
        let s = String::from("7_28]");
        let expected = String::from("7_28]");
        let result = Solution::reverse_only_letters(s);
        assert_eq!(result, expected);
    }
}
