#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        #[allow(dead_code)]
        fn get_valid_potions_binary_search(vec: &Vec<i32>, target: f64) -> i32 {
            let mut left: i64 = 0;
            let mut right = vec.len() as i64 - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                if (vec[mid as usize] as f64) < target {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            vec[(left as usize)..vec.len()].len() as i32
        }
        potions.sort();
        let mut result: Vec<i32> = Vec::with_capacity(spells.len());
        for s in spells {
            let target = success as f64 / s as f64;
            result.push(get_valid_potions_binary_search(&potions, target));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let spells = vec![5, 1, 3];
        let potions = vec![1, 2, 3, 4, 5];
        let success = 7;
        let expected = vec![4, 0, 3];
        let result1 = Solution::successful_pairs(spells.clone(), potions.clone(), success);
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_02() {
        let spells = vec![3, 1, 2];
        let potions = vec![8, 5, 8];
        let success = 16;
        let expected = vec![2, 0, 2];
        let result1 = Solution::successful_pairs(spells.clone(), potions.clone(), success);
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_03() {
        let spells = vec![
            40, 11, 24, 28, 40, 22, 26, 38, 28, 10, 31, 16, 10, 37, 13, 21, 9, 22, 21, 18, 34, 2,
            40, 40, 6, 16, 9, 14, 14, 15, 37, 15, 32, 4, 27, 20, 24, 12, 26, 39, 32, 39, 20, 19,
            22, 33, 2, 22, 9, 18, 12, 5,
        ];
        let potions = vec![
            31, 40, 29, 19, 27, 16, 25, 8, 33, 25, 36, 21, 7, 27, 40, 24, 18, 26, 32, 25, 22, 21,
            38, 22, 37, 34, 15, 36, 21, 22, 37, 14, 31, 20, 36, 27, 28, 32, 21, 26, 33, 37, 27, 39,
            19, 36, 20, 23, 25, 39, 40,
        ];
        let success = 600;
        let expected = vec![
            48, 0, 32, 37, 48, 22, 33, 47, 37, 0, 43, 6, 0, 46, 0, 21, 0, 22, 21, 14, 46, 0, 48,
            48, 0, 6, 0, 0, 0, 3, 46, 3, 45, 0, 34, 20, 32, 0, 33, 47, 45, 47, 20, 18, 22, 45, 0,
            22, 0, 14, 0, 0,
        ];
        let result1 = Solution::successful_pairs(spells.clone(), potions.clone(), success);
        assert_eq!(result1, expected);
    }
}
