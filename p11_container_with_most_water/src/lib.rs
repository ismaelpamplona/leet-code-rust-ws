struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut max_area = 0;
        while l < r {
            let width = (r - l) as i32;
            let min_height = height[r].min(height[l]);
            let cur_area = width * min_height;
            max_area = max_area.max(cur_area);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        max_area
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = Solution::max_area(height);
        assert_eq!(result, 49);
    }

    #[test]
    fn case_02() {
        let height = vec![1, 1];
        let result = Solution::max_area(height);
        assert_eq!(result, 1);
    }
}
