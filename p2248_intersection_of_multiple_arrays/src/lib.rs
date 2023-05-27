use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            for j in 0..nums[i].len() {
                let entry = map.entry(nums[i][j]).or_insert(0);
                *entry += 1;
            }
        }

        for k in map.keys() {
            if let Some(v) = map.get(k) {
                if *v == nums.len() as i32 {
                    result.push(*k);
                }
            }
        }
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![vec![3, 1, 2, 4, 5], vec![1, 2, 3, 4], vec![3, 4, 5, 6]];
        let result = Solution::intersection(nums);
        let expected = vec![3, 4];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let nums = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let result = Solution::intersection(nums);
        let expected = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_03() {
        let nums = vec![vec![4, 43, 15, 30, 27, 22], vec![15, 30, 43, 27, 10, 4]];
        let result = Solution::intersection(nums);
        let expected = vec![4, 15, 27, 30, 43];
        assert_eq!(result, expected);
    }
}
