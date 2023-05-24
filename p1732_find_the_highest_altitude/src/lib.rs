struct Solution;
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut curr = 0;
        let mut highest = 0;

        for i in 0..gain.len() {
            curr += gain[i];
            highest = highest.max(curr);
        }
        highest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let gain = vec![-5, 1, 5, 0, -7];
        let result = Solution::largest_altitude(gain);
        assert_eq!(result, 1);
    }

    #[test]
    fn case_02() {
        let gain = vec![-4, -3, -2, -1, 4, 3, 2];
        let result = Solution::largest_altitude(gain);
        assert_eq!(result, 0);
    }

    #[test]
    fn case_03() {
        let gain = vec![
            44, 32, -9, 52, 23, -50, 50, 33, -84, 47, -14, 84, 36, -62, 37, 81, -36, -85, -39, 67,
            -63, 64, -47, 95, 91, -40, 65, 67, 92, -28, 97, 100, 81,
        ];
        let result = Solution::largest_altitude(gain);
        assert_eq!(result, 781);
    }
}
