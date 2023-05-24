struct Solution;
impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let sv: Vec<char> = s.chars().collect();
        let tv: Vec<char> = t.chars().collect();
        let mut sum_cost = 0;
        let mut max_len = 0;
        let mut curr_len = 0;
        let mut left = 0;

        for right in 0..sv.len() {
            sum_cost += (sv[right] as i32 - tv[right] as i32).abs();
            curr_len += 1;

            while sum_cost > max_cost {
                sum_cost -= (sv[left] as i32 - tv[left] as i32).abs();
                curr_len -= 1;
                left += 1;
            }

            max_len = max_len.max(curr_len);
        }

        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let s = String::from("abcd");
        let t = String::from("bcdf");
        let max_cost = 3;
        let result = Solution::equal_substring(s, t, max_cost);
        assert_eq!(result, 3);
    }

    #[test]
    fn case_02() {
        let s = String::from("abcd");
        let t = String::from("cdef");
        let max_cost = 3;
        let result = Solution::equal_substring(s, t, max_cost);
        assert_eq!(result, 1);
    }

    #[test]
    fn case_03() {
        let s = String::from("abcd");
        let t = String::from("acde");
        let max_cost = 0;
        let result = Solution::equal_substring(s, t, max_cost);
        assert_eq!(result, 1);
    }
}
