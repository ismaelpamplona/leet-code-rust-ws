use std::cmp;

struct Solution;
impl Solution {
    pub fn find_length(nums: &Vec<i32>, k: i32) -> i32 {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let vec = vec![3, 1, 2, 7];
        let result = Solution::find_length(&vec, 8);
        assert_eq!(result, 3);
    }

    #[test]
    fn case_02() {
        let vec = vec![3, 1, 2, 7, 4, 2, 1, 1, 5];
        let result = Solution::find_length(&vec, 8);
        assert_eq!(result, 4);
    }

    #[test]
    fn case_03() {
        let vec = vec![3, 1, 2, 7, 4, 2, 1, 1, 5];
        let expected = vec![4, 2, 1, 1];
        let result = Solution::find_longest_sw(&vec, 8);
        assert_eq!(result, expected);
    }
}
