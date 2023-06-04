use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn maximum_sum_arr(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut ans = -1;

        for i in 0..nums.len() {
            fn get_digit_sum(mut num: i32) -> i32 {
                let mut digit_sum = 0;
                while num > 0 {
                    digit_sum += num % 10;
                    num /= 10;
                }
                digit_sum
            }

            let digit_sum = get_digit_sum(nums[i]);

            let entry = map.entry(digit_sum).or_insert(vec![]);
            entry.push(nums[i]);

            if entry.len() == 2 {
                let sum = entry[0] + entry[1];
                ans = ans.max(sum);
                if entry[0] < entry[1] {
                    entry.remove(0);
                } else {
                    entry.remove(1);
                }
            }
        }

        ans
    }

    pub fn maximum_sum_num(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans = -1;

        fn get_digit_sum(mut num: i32) -> i32 {
            let mut digit_sum = 0;
            while num > 0 {
                digit_sum += num % 10;
                num /= 10;
            }
            digit_sum
        }

        for i in 0..nums.len() {
            let digit_sum = get_digit_sum(nums[i]);
            let entry = map.entry(digit_sum).or_insert(0);
            let entry_value = *entry;

            if entry_value != 0 {
                ans = ans.max(nums[i] + entry_value);
            }

            *entry = entry_value.max(nums[i]);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![18, 43, 36, 13, 7];
        let result = Solution::maximum_sum_arr(nums.clone());
        let result2 = Solution::maximum_sum_num(nums.clone());
        assert_eq!(result, 54);
        assert_eq!(result2, 54);
    }

    #[test]
    fn case_02() {
        let nums = vec![10, 12, 19, 14];
        let result = Solution::maximum_sum_arr(nums.clone());
        let result2 = Solution::maximum_sum_num(nums.clone());
        assert_eq!(result, -1);
        assert_eq!(result2, -1);
    }

    #[test]
    fn case_03() {
        let nums = vec![18, 43, 36, 13, 7, 27];
        let result = Solution::maximum_sum_arr(nums.clone());
        let result2 = Solution::maximum_sum_num(nums.clone());
        assert_eq!(result, 63);
        assert_eq!(result2, 63);
    }

    #[test]
    fn case_04() {
        let nums = vec![
            368, 369, 307, 304, 384, 138, 90, 279, 35, 396, 114, 328, 251, 364, 300, 191, 438, 467,
            183,
        ];
        let result = Solution::maximum_sum_arr(nums.clone());
        let result2 = Solution::maximum_sum_num(nums.clone());
        assert_eq!(result, 835);
        assert_eq!(result2, 835);
    }

    #[test]
    fn case_05() {
        let nums = vec![23, 14, 5, 32];
        let result = Solution::maximum_sum_arr(nums.clone());
        let result2 = Solution::maximum_sum_num(nums.clone());
        assert_eq!(result, 55);
        assert_eq!(result2, 55);
    }
}
