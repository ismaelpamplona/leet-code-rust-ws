use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::from_iter(nums);
        let mut longest = 0;
        for n in &set {
            if !set.contains(&(n - 1)) {
                let mut cur_num = *n;
                let mut cur_len = 1;
                while set.contains(&(cur_num + 1)) {
                    cur_num += 1;
                    cur_len += 1;
                }
                longest = longest.max(cur_len);
            }
        }
        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        let result = Solution::longest_consecutive(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn case_02() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let result = Solution::longest_consecutive(nums);
        assert_eq!(result, 9);
    }
}
