struct Solution;
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut running_sum: Vec<i32> = vec![];
        let mut curr_sum = 0;
        for n in nums {
            curr_sum = n + curr_sum;
            running_sum.push(curr_sum);
        }
        running_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::running_sum(nums);
        assert_eq!(result, vec![1, 3, 6, 10]);
    }

    #[test]
    fn case_2() {
        let nums = vec![1, 1, 1, 1, 1];
        let result = Solution::running_sum(nums);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn case_3() {
        let nums = vec![3, 1, 2, 10, 1];
        let result = Solution::running_sum(nums);
        assert_eq!(result, vec![3, 4, 6, 16, 17]);
    }
}
