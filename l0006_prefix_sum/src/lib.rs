struct Solution;
impl Solution {
    pub fn prefix_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix = vec![nums[0]];

        for i in 1..nums.len() {
            prefix.push(nums[i] + prefix[prefix.len() - 1]);
        }

        prefix

        // time complexity: O(n)
        // space complexity: O(n)
    }

    pub fn prefix_sum_2(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix = vec![];
        let mut sum = 0;

        for i in 0..nums.len() {
            sum += nums[i];
            prefix.push(sum);
        }

        prefix

        // time complexity: O(n)
        // space complexity: O(n)
    }

    pub fn answer_queries(nums: Vec<i32>, queries: Vec<(i32, i32)>, limit: i32) -> Vec<bool> {
        let mut prefix = vec![nums[0]];
        for i in 1..nums.len() {
            prefix.push(nums[i] + prefix[i - 1]);
        }

        let mut result = vec![];
        for j in 0..queries.len() {
            let right = queries[j].1 as usize;
            let left = queries[j].0 as usize;
            let sum = prefix[right] - prefix[left] + nums[left];
            result.push(sum < limit);
        }

        result

        // n = nums.len()
        // time complexity: O(n)
        // space complexity: O(n)
    }

    pub fn answer_queries_2(nums: Vec<i32>, queries: Vec<(i32, i32)>, limit: i32) -> Vec<bool> {
        let mut result = vec![];
        for i in 0..queries.len() {
            let mut sum = 0;
            for j in queries[i].0..=queries[i].1 {
                sum += nums[j as usize];
            }

            result.push(sum < limit);
        }
        result

        // n = nums.len()
        // m = queries.len()
        // time complexity: O(n*m)
        // space complexity: O(n)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![5, 2, 1, 6, 3, 8];
        let expected = vec![5, 7, 8, 14, 17, 25];
        let result = Solution::prefix_sum(nums.clone());
        let result2 = Solution::prefix_sum_2(nums);
        assert_eq!(result, expected);
        assert_eq!(result2, expected);
    }

    #[test]
    fn case_02() {
        let nums = vec![1, 6, 3, 2, 7, 2];
        let queries = vec![(0, 3), (2, 5), (2, 4)];
        let expected = vec![true, false, true];
        let result = Solution::answer_queries(nums.clone(), queries.clone(), 13);
        let result2 = Solution::answer_queries_2(nums.clone(), queries.clone(), 13);
        assert_eq!(result, expected);
        assert_eq!(result2, expected);
    }
}
