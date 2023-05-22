struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut left = 0;
        let mut right = 0;
        let mut str_vec: Vec<char> = vec![];

        for (i, c) in s.char_indices() {
            str_vec.push(c);
            if c == ' ' {
                right = i - 1;
            }

            if i == s.len() - 1 {
                right = i;
            }

            while left < right {
                let tmp = str_vec[left];
                str_vec[left] = str_vec[right];
                str_vec[right] = tmp;
                right -= 1;
                left += 1;
            }

            if c == ' ' {
                left = i + 1;
                right = i + 1;
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
        let s = String::from("Let's take LeetCode contest");
        let expected = String::from("s'teL ekat edoCteeL tsetnoc");
        let result = Solution::reverse_words(s);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let s = String::from("God Ding");
        let expected = String::from("doG gniD");
        let result = Solution::reverse_words(s);
        assert_eq!(result, expected);
    }
}
