use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }
        let map: HashMap<char, char> = vec![('(', ')'), ('{', '}'), ('[', ']')]
            .into_iter()
            .collect();
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if map.contains_key(&c) {
                stack.push(c);
            } else {
                if stack.is_empty() {
                    return false;
                }
                let open = stack.pop().unwrap();
                let close = map.get(&open).unwrap();
                if *close != c {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let s = String::from("()");
        let result = Solution::is_valid(s.clone());
        assert_eq!(result, true);
    }

    #[test]
    fn case_02() {
        let s = String::from("()[]{}");
        let result = Solution::is_valid(s.clone());
        assert_eq!(result, true);
    }

    #[test]
    fn case_03() {
        let s = String::from("(]");
        let result = Solution::is_valid(s.clone());
        assert_eq!(result, false);
    }

    #[test]
    fn case_04() {
        let s = String::from("{[]}");
        let result = Solution::is_valid(s.clone());
        assert_eq!(result, true);
    }

    #[test]
    fn case_05() {
        let s = String::from("([)]");
        let result = Solution::is_valid(s.clone());
        assert_eq!(result, false);
    }
}
