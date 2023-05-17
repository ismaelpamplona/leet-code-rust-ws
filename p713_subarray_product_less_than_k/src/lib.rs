struct Solution;
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }

        let (mut ans, mut curr_product, mut left) = (0, 1, 0);

        for right in 0..nums.len() {
            curr_product *= nums[right];

            while curr_product >= k {
                curr_product /= nums[left];
                left += 1;
            }

            ans += right - left + 1;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let vec = vec![10, 5, 2, 6];
        let result = Solution::num_subarray_product_less_than_k(vec, 100);
        assert_eq!(result, 8);
    }

    #[test]
    fn case_02() {
        let vec = vec![1, 2, 3];
        let result = Solution::num_subarray_product_less_than_k(vec, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn case_03() {
        let vec = vec![1, 1, 1];
        let result = Solution::num_subarray_product_less_than_k(vec, 1);
        assert_eq!(result, 0);
    }

    #[test]
    fn case_04() {
        let vec = vec![0, 0, 0];
        let result = Solution::num_subarray_product_less_than_k(vec, 2);
        assert_eq!(result, 6);
    }
}
