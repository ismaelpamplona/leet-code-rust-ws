struct Solution;
impl Solution {
    fn backtrack(result: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>, remain: i32, k: i32, i: i32) {
        if remain == 0 && cur.len() as i32 == k {
            result.push(cur.to_vec());
            return;
        } else if remain < 0 || cur.len() as i32 == k {
            return;
        }
        for num in i..=9 {
            cur.push(num);
            Self::backtrack(result, cur, remain - num, k, num + 1);
            cur.pop();
        }
    }
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::backtrack(&mut result, &mut vec![], n, k, 1);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let result = Solution::combination_sum3(3, 7);
        let expected = vec![vec![1, 2, 4]];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let result = Solution::combination_sum3(3, 9);
        let expected = vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_03() {
        let result = Solution::combination_sum3(4, 1);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }
}
