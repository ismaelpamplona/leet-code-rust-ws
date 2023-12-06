struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let mut memo = vec![0; amount as usize];
        Self::dp(&coins, amount, &mut memo)
    }

    fn dp(coins: &Vec<i32>, rem: i32, memo: &mut Vec<i32>) -> i32 {
        if rem < 0 {
            return -1;
        }
        if rem == 0 {
            return 0;
        }
        if memo[(rem - 1) as usize] != 0 {
            return memo[(rem - 1) as usize];
        }
        let mut min = i32::MAX;
        for &coin in coins.iter() {
            let res = Self::dp(coins, rem - coin, memo);
            if res >= 0 && res < min {
                min = 1 + res;
            }
        }
        memo[(rem - 1) as usize] = if min == i32::MAX { -1 } else { min };
        memo[(rem - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let coins = vec![1, 2, 5];
        let result1 = Solution::coin_change(coins.clone(), 11);
        assert_eq!(result1, 3);
    }

    #[test]
    fn case_02() {
        let coins = vec![2];
        let result1 = Solution::coin_change(coins.clone(), 3);
        assert_eq!(result1, -1);
    }

    #[test]
    fn case_03() {
        let coins = vec![1];
        let result1 = Solution::coin_change(coins.clone(), 0);
        assert_eq!(result1, 0);
    }
}
