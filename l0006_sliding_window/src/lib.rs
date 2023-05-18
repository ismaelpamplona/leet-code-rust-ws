use std::cmp;

struct Solution;
impl Solution {
    pub fn find_length_sums_k(nums: &Vec<i32>, k: i32) -> i32 {
        let (mut left, mut curr, mut ans) = (0, 0, 0);

        for right in 0..nums.len() {
            curr += nums[right];

            while curr > k {
                curr -= nums[left];
                left += 1;
            }

            let size = right - left + 1;
            ans = cmp::max(ans, size);
        }

        ans as i32

        // time complexity O(n)
        // space complexity O(1)
    }

    pub fn find_longest_sw(nums: &Vec<i32>, k: i32) -> Vec<i32> {
        let (mut left, mut curr) = (0, 0);

        let (mut biggest_sw, mut current_sw) = (vec![], vec![]);

        for right in 0..nums.len() {
            curr += nums[right];
            current_sw.push(nums[right]);

            while curr > k {
                curr -= nums[left];
                left += 1;
                current_sw.remove(0);
            }

            if current_sw.len() > biggest_sw.len() {
                biggest_sw = current_sw.clone();
            }
        }

        biggest_sw

        // time complexity O(n)
        // space complexity O(n)
    }

    pub fn find_length_1_seq(s: &str) -> i32 {
        let char_vec: Vec<char> = s.chars().collect();

        let (mut zero_count, mut left, mut ans) = (0, 0, 0);

        for right in 0..char_vec.len() {
            if char_vec[right] == '0' {
                zero_count += 1;
            }

            while zero_count > 1 {
                if char_vec[left] == '0' {
                    zero_count -= 1;
                }
                left += 1;
            }

            let size = right - left + 1;
            ans = cmp::max(ans, size);
        }

        ans as i32

        // time complexity O(n)
        // space complexity O(1)
    }

    pub fn find_largest_sum_and_length_k(nums: &Vec<i32>, k: i32) -> i32 {
        let (mut curr, mut sum, mut left) = (0, 0, 0);

        for right in 0..nums.len() {
            curr += nums[right];
            if right as i32 >= k {
                curr -= nums[left];
                left += 1;
            }
            sum = cmp::max(sum, curr);
        }

        sum

        // time complexity O(n)
        // space complexity O(1)
    }

    pub fn find_largest_sum_and_length_k_2(nums: &Vec<i32>, k: usize) -> i32 {
        let (mut curr, mut sum) = (0, 0);

        for i in 0..k {
            curr += nums[i];
        }

        sum = curr;

        for right in k..nums.len() {
            curr += nums[right] - nums[right - k];
            sum = cmp::max(sum, curr);
        }

        sum

        // time complexity O(n)
        // space complexity O(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let vec = vec![3, 1, 2, 7];
        let result = Solution::find_length_sums_k(&vec, 8);
        assert_eq!(result, 3);
    }

    #[test]
    fn case_02() {
        let vec = vec![3, 1, 2, 7, 4, 2, 1, 1, 5];
        let result = Solution::find_length_sums_k(&vec, 8);
        assert_eq!(result, 4);
    }

    #[test]
    fn case_03() {
        let vec = vec![3, 1, 2, 7, 4, 2, 1, 1, 5];
        let expected = vec![4, 2, 1, 1];
        let result = Solution::find_longest_sw(&vec, 8);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_04() {
        let bin = "1101100111";
        let result = Solution::find_length_1_seq(bin);
        assert_eq!(result, 5);
    }

    #[test]
    fn case_05() {
        let bin = "1100100111";
        let result = Solution::find_length_1_seq(bin);
        assert_eq!(result, 4);
    }

    #[test]
    fn case_06() {
        let bin: &str = "111111100";
        let result = Solution::find_length_1_seq(bin);
        assert_eq!(result, 8);
    }

    #[test]
    fn case_07() {
        let bin: &str = "0111111100";
        let result = Solution::find_length_1_seq(bin);
        assert_eq!(result, 8);
    }

    #[test]
    fn case_08() {
        let bin: &str = "00111111100";
        let result = Solution::find_length_1_seq(bin);
        assert_eq!(result, 8);
    }

    #[test]
    fn case_09() {
        let bin: &str = "00000";
        let result = Solution::find_length_1_seq(bin);
        assert_eq!(result, 1);
    }

    #[test]
    fn case_10() {
        let bin: &str = "000001";
        let result = Solution::find_length_1_seq(bin);
        assert_eq!(result, 2);
    }

    #[test]
    fn case_11() {
        let vec = vec![3, -1, 4, 12, -8, 5, 6];
        let result = Solution::find_largest_sum_and_length_k(&vec, 4);
        assert_eq!(result, 18);
    }

    #[test]
    fn case_12() {
        let vec = vec![3, -1, 4, 12, -8, 5, 6];
        let result = Solution::find_largest_sum_and_length_k_2(&vec, 4);
        assert_eq!(result, 18);
    }
}
