struct Solution;
impl Solution {
    pub fn trap_a(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut max_left = vec![0; n];
        let mut max_right = vec![0; n];
        let mut cur_max_left = 0;
        let mut cur_max_right = 0;
        for i in 1..n {
            cur_max_left = cur_max_left.max(height[i - 1]);
            max_left[i] = cur_max_left;
            cur_max_right = cur_max_right.max(height[n - i]);
            max_right[n - i - 1] = cur_max_right;
        }
        let mut min_left_right = vec![0; n];
        let mut max_vol = 0;
        for i in 0..n {
            min_left_right[i] = max_left[i].min(max_right[i]);
            let max_trap = min_left_right[i] - height[i];
            if max_trap >= 0 {
                max_vol += max_trap;
            }
        }
        max_vol
    }

    pub fn trap_b(height: Vec<i32>) -> i32 {
        let n = height.len() - 1;
        if n <= 0 {
            return 0;
        }
        let mut l = 0;
        let mut r = n;
        let mut max_left = height[l];
        let mut max_right = height[r];
        let mut max_vol = 0;
        while l < r {
            if max_left < max_right {
                l += 1;
                max_left = max_left.max(height[l]);
                max_vol += max_left - height[l];
            } else {
                r -= 1;
                max_right = max_right.max(height[r]);
                max_vol += max_right - height[r];
            }
        }
        max_vol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let result1 = Solution::trap_a(height.clone());
        let result2 = Solution::trap_b(height.clone());
        assert_eq!(result1, 6);
        assert_eq!(result2, 6);
    }

    #[test]
    fn case_02() {
        let height = vec![4, 2, 0, 3, 2, 5];
        let result1 = Solution::trap_a(height.clone());
        let result2 = Solution::trap_b(height.clone());
        assert_eq!(result1, 9);
        assert_eq!(result2, 9);
    }
}
