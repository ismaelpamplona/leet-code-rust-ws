struct Solution;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        nums.sort();
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let trhee_sum = nums[i] + nums[left] + nums[right];
                if trhee_sum > 0 {
                    right -= 1;
                } else if trhee_sum < 0 {
                    left += 1;
                } else {
                    ans.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        let result = Solution::three_sum(nums);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let nums = vec![0, 1, 1];
        let expected: Vec<Vec<i32>> = vec![];
        let result = Solution::three_sum(nums);
        assert_eq!(result, expected);
    }
}
