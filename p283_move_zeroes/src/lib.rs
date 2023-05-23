struct Solution;
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut last_non_zero_found_at = 0;

        for cur in 0..nums.len() {
            if nums[cur] != 0 {
                nums.swap(last_non_zero_found_at, cur);
                last_non_zero_found_at += 1;
            }
        }

        // optimal
        // time complexity: O(n)
        // space complexity: O(1)
    }

    pub fn move_zeroes_2(nums: &mut Vec<i32>) {
        let mut zero_count = 0;
        let mut i: i32 = 0;

        while i < nums.len() as i32 {
            if nums[i as usize] == 0 {
                zero_count += 1;
                nums.remove(i as usize);
                i -= 1;
            }
            i += 1;
        }

        for _ in 0..zero_count {
            nums.push(0);
        }

        // sub-optimal
        // time complexity: O(n)
        // space complexity: O(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let mut nums = vec![0, 1, 0, 3, 12];
        let mut nums2 = vec![0, 1, 0, 3, 12];
        let expected = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut nums);
        Solution::move_zeroes_2(&mut nums2);
        assert_eq!(nums, expected);
        assert_eq!(nums2, expected);
    }

    #[test]
    fn case_02() {
        let mut nums = vec![0];
        let mut nums2 = vec![0];
        let expected = vec![0];
        Solution::move_zeroes(&mut nums);
        Solution::move_zeroes(&mut nums);
        Solution::move_zeroes_2(&mut nums2);
        assert_eq!(nums, expected);
        assert_eq!(nums2, expected);
    }

    #[test]
    fn case_03() {
        let mut nums = vec![0, 0, 1];
        let expected = vec![1, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }
}
