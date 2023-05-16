struct Solution;
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let size = nums.len();
        let mut result = vec![0; size];
        let (mut left, mut right) = (0, size - 1);

        for i in (0..size).rev() {
            let mut number: i32;
            if nums[left].abs() > nums[right].abs() {
                number = nums[left];
                left += 1;
            } else {
                number = nums[right];
                right -= 1;
            }
            result[i] = number * number;
            // result[i] = i32::pow(number, 2);
        }

        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let input = vec![-4, -1, 0, 3, 10];
        let output = vec![0, 1, 9, 16, 100];
        let result = Solution::sorted_squares(input);
        assert_eq!(result, output);
    }

    #[test]
    fn case_02() {
        let input = vec![-7, -3, 2, 3, 11];
        let output = vec![4, 9, 9, 49, 121];
        let result = Solution::sorted_squares(input);
        assert_eq!(result, output);
    }

    #[test]
    fn case_03() {
        let input = vec![-5, -3, -2, -1];
        let output = vec![1, 4, 9, 25];
        let result = Solution::sorted_squares(input);
        assert_eq!(result, output);
    }
}
