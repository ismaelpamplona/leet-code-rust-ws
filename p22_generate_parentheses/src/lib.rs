struct Solution;
impl Solution {
    fn backtrack(result: &mut Vec<String>, cur: &mut Vec<char>, left: i32, right: i32, n: i32) {
        if cur.len() as i32 == 2 * n {
            result.push(cur.iter().collect());
            return;
        }

        if left < n {
            cur.push('(');
            Self::backtrack(result, cur, left + 1, right, n);
            cur.pop();
        }

        if left > right {
            cur.push(')');
            Self::backtrack(result, cur, left, right + 1, n);
            cur.pop();
        }
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        Self::backtrack(&mut result, &mut vec![], 0, 0, n);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let result = Solution::generate_parenthesis(3);
        let expected = [
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let result = Solution::generate_parenthesis(1);
        let expected = ["()".to_string()];
        assert_eq!(result, expected);
    }
}
