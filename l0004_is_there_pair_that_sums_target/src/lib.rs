struct Solution;
impl Solution {
    pub fn if_there_pair_that_sums_target(collection: Vec<i32>, target: i32) -> bool {
        let mut left = 0 as usize;
        let mut right = collection.len() - 1;
        let mut sum = collection[left] + collection[right];

        while sum != target && left < right {
            if sum > target {
                right -= 1;
            } else {
                left += 1;
            }
            sum = collection[left] + collection[right];
            if sum == target {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let collection = vec![1, 2, 4, 6, 8, 9, 14, 15];
        let target = 13;
        let result = Solution::if_there_pair_that_sums_target(collection, target);
        assert_eq!(result, true);
    }

    #[test]
    fn case_02() {
        let collection = vec![1, 2, 4];
        let target = 13;
        let result = Solution::if_there_pair_that_sums_target(collection, target);
        assert_eq!(result, false);
    }

    #[test]
    fn case_03() {
        let collection = vec![1, 2, 4];
        let target = 6;
        let result = Solution::if_there_pair_that_sums_target(collection, target);
        assert_eq!(result, true);
    }
}
