use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        for i in 0..nums.len() {
            let entry = map.entry(nums[i]).or_insert(vec![]);
            entry.push(i as i32);
        }

        for i in 0..nums.len() {
            let diff = target - nums[i];

            if let Some(value) = map.get(&diff) {
                if value.len() > 1 {
                    return vec![i as i32, value[1]];
                } else if diff != nums[i] {
                    return vec![i as i32, value[0]];
                }
            }
        }

        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let s = vec![2, 7, 11, 15];
        let result = Solution::two_sum(s, 9);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn case_02() {
        let s = vec![3, 2, 4];
        let result = Solution::two_sum(s, 6);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn case_03() {
        let s = vec![3, 3];
        let result = Solution::two_sum(s, 6);
        assert_eq!(result, vec![0, 1]);
    }
}
