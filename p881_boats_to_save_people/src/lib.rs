use std::f32::consts::PI;

struct Solution;
impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort();
        let (mut left, mut right) = (0, people.len() as i32 - 1);
        let mut boats = 0;
        while left <= right {
            if people[left as usize] + people[right as usize] <= limit {
                left += 1;
            }
            right -= 1;
            boats += 1;
        }
        boats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let people = vec![1, 2];
        let limit = 3;
        let result1 = Solution::num_rescue_boats(people.clone(), limit);
        assert_eq!(result1, 1);
    }

    #[test]
    fn case_02() {
        let people = vec![3, 2, 2, 1];
        let limit = 3;
        let result1 = Solution::num_rescue_boats(people.clone(), limit);
        assert_eq!(result1, 3);
    }

    #[test]
    fn case_03() {
        let people = vec![3, 5, 3, 4];
        let limit = 5;
        let result = Solution::num_rescue_boats(people, limit);
        assert_eq!(result, 4);
    }
}
