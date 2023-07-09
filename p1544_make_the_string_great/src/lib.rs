struct Solution;
impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            let len = stack.len();
            if c.is_ascii_uppercase()
                && len > 0
                && stack[len - 1].to_string() == c.to_lowercase().to_string()
            {
                stack.pop();
            } else if c.is_ascii_lowercase()
                && len > 0
                && stack[len - 1].to_string() == c.to_uppercase().to_string()
            {
                stack.pop();
            } else {
                stack.push(c)
            }
        }
        stack.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let s = String::from("leEeetcode");
        let output = String::from("leetcode");
        let result = Solution::make_good(s);
        assert_eq!(result, output);
    }

    #[test]
    fn case_02() {
        let s = String::from("abBAcC");
        let output = String::from("");
        let result = Solution::make_good(s);
        assert_eq!(result, output);
    }

    #[test]
    fn case_03() {
        let s = String::from("s");
        let output = String::from("s");
        let result = Solution::make_good(s);
        assert_eq!(result, output);
    }
}
