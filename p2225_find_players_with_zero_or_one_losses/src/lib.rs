use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut no_lost: Vec<i32> = vec![];
        let mut one_lost: Vec<i32> = vec![];
        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in 0..matches.len() {
            let winner = matches[i][0];
            let loser = matches[i][1];

            map.entry(winner).or_insert(0);

            let loser_entry = map.entry(loser).or_insert(0);
            *loser_entry += 1;
        }

        for (key, value) in &map {
            if value == &0 {
                no_lost.push(*key);
            } else if value == &1 {
                one_lost.push(*key);
            }
        }
        no_lost.sort();
        one_lost.sort();

        vec![no_lost, one_lost]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let matches = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];
        let result = Solution::find_winners(matches);
        let expected = vec![vec![1, 2, 10], vec![4, 5, 7, 8]];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let matches = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];
        let result = Solution::find_winners(matches);
        let expected = vec![vec![1, 2, 5, 6], vec![]];
        assert_eq!(result, expected);
    }
}
