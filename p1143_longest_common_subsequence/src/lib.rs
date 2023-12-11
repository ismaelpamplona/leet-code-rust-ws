struct Solution;
impl Solution {
    pub fn longest_common_subsequence_topdown(text1: String, text2: String) -> i32 {
        let t1: Vec<char> = text1.chars().collect();
        let m = t1.len();
        let t2: Vec<char> = text2.chars().collect();
        let n = t2.len();
        let mut memo: Vec<Vec<i32>> = vec![vec![-1; n + 1]; m + 1];
        Solution::dp(0, 0, &t1, &t2, m, n, &mut memo)
    }
    fn dp(
        i: usize,
        j: usize,
        t1: &Vec<char>,
        t2: &Vec<char>,
        m: usize,
        n: usize,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if i >= m || j >= n {
            return 0;
        }

        if memo[i][j] != -1 {
            return memo[i][j];
        }

        let mut ans = 0;
        if t1[i] == t2[j] {
            ans = 1 + Solution::dp(i + 1, j + 1, t1, t2, m, n, memo);
        } else {
            let ans1 = Solution::dp(i + 1, j, t1, t2, m, n, memo);
            let ans2 = Solution::dp(i, j + 1, t1, t2, m, n, memo);
            ans = ans1.max(ans2);
        }

        memo[i][j] = ans;

        ans
    }

    pub fn longest_common_subsequence_bup(text1: String, text2: String) -> i32 {
        let t1: Vec<char> = text1.chars().collect();
        let m = t1.len();
        let t2: Vec<char> = text2.chars().collect();
        let n = t2.len();
        let mut memo = vec![vec![0; n + 1]; m + 1];

        for i in 0..m {
            for j in 0..n {
                if t1[i] == t2[j] {
                    memo[i + 1][j + 1] = 1 + memo[i][j];
                } else {
                    memo[i + 1][j + 1] = memo[i + 1][j].max(memo[i][j + 1]);
                }
            }
        }

        memo[m][n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let text1 = String::from("abcde");
        let text2 = String::from("ace");
        let result1 = Solution::longest_common_subsequence_topdown(text1.clone(), text2.clone());
        let result2 = Solution::longest_common_subsequence_bup(text1.clone(), text2.clone());
        assert_eq!(result1, 3);
        assert_eq!(result2, 3);
    }

    #[test]
    fn case_02() {
        let text1 = String::from("abc");
        let text2 = String::from("abc");
        let result1 = Solution::longest_common_subsequence_topdown(text1.clone(), text2.clone());
        let result2 = Solution::longest_common_subsequence_bup(text1.clone(), text2.clone());
        assert_eq!(result1, 3);
        assert_eq!(result2, 3);
    }

    #[test]
    fn case_03() {
        let text1 = String::from("abc");
        let text2 = String::from("def");
        let result1 = Solution::longest_common_subsequence_topdown(text1.clone(), text2.clone());
        let result2 = Solution::longest_common_subsequence_bup(text1.clone(), text2.clone());
        assert_eq!(result1, 0);
        assert_eq!(result2, 0);
    }
}
