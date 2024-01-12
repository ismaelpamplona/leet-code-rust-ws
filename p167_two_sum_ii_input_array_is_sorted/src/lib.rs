struct Solution;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;
        while left < right {
            let sum = numbers[left] + numbers[right];
            if sum < target {
                left += 1;
            } else if sum > target {
                right -= 1;
            } else {
                return vec![(left + 1) as i32, (right + 1) as i32];
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let numbers = vec![2, 7, 11, 15];
        let expected = vec![1, 2];
        let result = Solution::two_sum(numbers, 9);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let numbers = vec![2, 3, 4];
        let expected = vec![1, 3];
        let result = Solution::two_sum(numbers, 6);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_03() {
        let numbers = vec![-1, 0];
        let expected = vec![1, 2];
        let result = Solution::two_sum(numbers, -1);
        assert_eq!(result, expected);
    }
}
