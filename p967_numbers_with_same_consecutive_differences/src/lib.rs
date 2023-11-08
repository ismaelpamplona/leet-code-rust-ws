struct Solution;
impl Solution {
    fn backtrack(result: &mut Vec<i32>, cur: &mut Vec<i32>, n: i32, k: i32, num: i32) {
        if cur.len() as i32 == n {
            let mut num = 0;
            for &digit in cur.iter() {
                num = num * 10 + digit;
            }
            result.push(num);
            return;
        }
        let next1 = num + k;
        let next2 = num - k;

        if next1 >= 0 && next1 <= 9 {
            cur.push(next1);
            Self::backtrack(result, cur, n, k, next1);
            cur.pop();
        }

        if k != 0 && next1 != next2 && next2 >= 0 && next2 <= 9 {
            cur.push(next2);
            Self::backtrack(result, cur, n, k, next2);
            cur.pop();
        }
    }

    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut result = vec![];
        for num in 1..=9 {
            let mut cur = vec![num];
            Self::backtrack(&mut result, &mut cur, n, k, num);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let mut result = Solution::nums_same_consec_diff(3, 7);
        result.sort();
        let expected = vec![181, 292, 707, 818, 929];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let mut result = Solution::nums_same_consec_diff(2, 1);
        result.sort();
        let expected = vec![
            10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_03() {
        let mut result = Solution::nums_same_consec_diff(2, 0);
        result.sort();
        let expected = vec![11, 22, 33, 44, 55, 66, 77, 88, 99];
        assert_eq!(result, expected);
    }
}
