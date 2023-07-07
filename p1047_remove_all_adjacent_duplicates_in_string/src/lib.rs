struct Solution;
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if stack.len() > 0 && stack[stack.len() - 1] == c {
                stack.pop();
            } else {
                stack.push(c);
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
        let s = String::from("abbaca");
        let output = String::from("ca");
        let result = Solution::remove_duplicates(s);
        assert_eq!(result, output);
    }

    #[test]
    fn case_02() {
        let s = String::from("azxxzy");
        let output = String::from("ay");
        let result = Solution::remove_duplicates(s);
        assert_eq!(result, output);
    }
}
