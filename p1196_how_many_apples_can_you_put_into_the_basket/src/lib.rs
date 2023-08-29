struct Solution;
impl Solution {
    pub fn max_number_of_apples(mut weight: Vec<i32>) -> i32 {
        weight.sort();
        let (mut count, mut cur_weight) = (0, 0);
        for apple in weight {
            cur_weight += apple;
            if cur_weight <= 5000 {
                count += 1;
            } else {
                break;
            }
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let weight = vec![100, 200, 150, 1000];
        let result1 = Solution::max_number_of_apples(weight);
        assert_eq!(result1, 4);
    }

    #[test]
    fn case_02() {
        let weight = vec![900, 950, 800, 1000, 700, 800];
        let result1 = Solution::max_number_of_apples(weight);
        assert_eq!(result1, 5);
    }
}
