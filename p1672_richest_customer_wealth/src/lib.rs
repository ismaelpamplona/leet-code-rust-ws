struct Solution;
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        for cus in accounts {
            let mut cus_wealth = 0;
            for val in cus {
                cus_wealth += val;
            }
            if cus_wealth > max {
                max = cus_wealth;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = vec![vec![1, 2, 3], vec![3, 2, 1]];
        let output = Solution::maximum_wealth(input);
        assert_eq!(output, 6);
    }

    #[test]
    fn case_2() {
        let input = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
        let output = Solution::maximum_wealth(input);
        assert_eq!(output, 10);
    }

    #[test]
    fn case_3() {
        let input = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
        let output = Solution::maximum_wealth(input);
        assert_eq!(output, 17);
    }
}
