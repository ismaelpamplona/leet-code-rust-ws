use std::collections::HashMap;
struct Solution;
impl Solution {
    fn dp(i: usize, questions: &Vec<Vec<i32>>, map: &mut HashMap<usize, i64>) -> i64 {
        if i >= questions.len() {
            return 0;
        }
        if let Some(&cached) = map.get(&i) {
            return cached;
        }

        let pts = questions[i][0] as i64;
        let skip = questions[i][1] as usize;

        let max = Self::dp(i + 1, questions, map).max(pts + Self::dp(i + 1 + skip, questions, map));
        map.insert(i, max);
        max
    }

    pub fn most_points_tdown(questions: Vec<Vec<i32>>) -> i64 {
        let mut map = HashMap::new();
        Self::dp(0, &questions, &mut map)
    }

    pub fn most_points_bup(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut map = HashMap::new();
        map.insert(n - 1, questions[n - 1][0] as i64);

        for i in (0..n - 1).rev() {
            let points = questions[i][0] as i64;
            let skip = questions[i][1] as usize;
            let mut value = points;
            if i + skip + 1 < n {
                value += *map.get(&(i + skip + 1)).unwrap_or(&0);
            }
            let skip_value = *map.get(&(i + 1)).unwrap_or(&0);
            map.insert(i, value.max(skip_value));
        }
        *map.get(&0).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let questions = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
        let result1 = Solution::most_points_tdown(questions.clone());
        let result2 = Solution::most_points_tdown(questions.clone());
        assert_eq!(result1, 5);
        assert_eq!(result2, 5);
    }

    #[test]
    fn case_02() {
        let questions = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];
        let result1 = Solution::most_points_tdown(questions.clone());
        let result2 = Solution::most_points_tdown(questions.clone());
        assert_eq!(result1, 7);
        assert_eq!(result2, 7);
    }

    #[test]
    fn case_03() {
        let questions = vec![
            vec![21, 5],
            vec![92, 3],
            vec![74, 2],
            vec![39, 4],
            vec![58, 2],
            vec![5, 5],
            vec![49, 4],
            vec![65, 3],
        ];
        let result1 = Solution::most_points_tdown(questions.clone());
        let result2 = Solution::most_points_tdown(questions.clone());
        assert_eq!(result1, 157);
        assert_eq!(result2, 157);
    }
}
