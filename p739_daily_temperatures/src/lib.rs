use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn daily_temperatures(temps: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0; temps.len()];
        let mut stack: VecDeque<i32> = VecDeque::new();
        for i in 0..temps.len() {
            while !stack.is_empty() && temps[stack[stack.len() - 1] as usize] < temps[i] {
                let j = stack.pop_back().unwrap();
                ans[j as usize] = i as i32 - j;
            }
            stack.push_back(i as i32);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let output = vec![1, 1, 4, 2, 1, 1, 0, 0];
        let result1 = Solution::daily_temperatures(temperatures.clone());
        assert_eq!(result1, output);
    }

    #[test]
    fn case_02() {
        let temperatures = vec![30, 40, 50, 60];
        let output = vec![1, 1, 1, 0];
        let result1 = Solution::daily_temperatures(temperatures.clone());
        assert_eq!(result1, output);
    }

    #[test]
    fn case_03() {
        let temperatures = vec![30, 60, 90];
        let output = vec![1, 1, 0];
        let result1 = Solution::daily_temperatures(temperatures.clone());
        assert_eq!(result1, output);
    }

    #[test]
    fn case_04() {
        let temperatures = vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70];
        let output = vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0];
        let result1 = Solution::daily_temperatures(temperatures.clone());
        assert_eq!(result1, output);
    }

    #[test]
    fn case_05() {
        let temperatures = vec![34, 33, 32, 31, 30, 50];
        let output = vec![5, 4, 3, 2, 1, 0];
        let result1 = Solution::daily_temperatures(temperatures.clone());
        assert_eq!(result1, output);
    }

    #[test]
    fn case_06() {
        let temperatures = vec![40, 35, 32, 37, 50];
        let output = vec![4, 2, 1, 1, 0];
        let result1 = Solution::daily_temperatures(temperatures.clone());
        assert_eq!(result1, output);
    }
}
