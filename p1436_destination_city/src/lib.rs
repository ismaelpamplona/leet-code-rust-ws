use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut set: HashSet<&String> = HashSet::new();
        let mut ans: String = String::from("");

        for i in 0..paths.len() {
            set.insert(&paths[i][0]);
        }

        for i in 0..paths.len() {
            if !set.contains(&paths[i][1]) {
                return paths[i][1].clone();
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let paths = vec![
            vec![String::from("London"), String::from("New York")],
            vec![String::from("New York"), String::from("Lima")],
            vec![String::from("Lima"), String::from("Sao Paulo")],
        ];
        let result1 = Solution::dest_city(paths);
        assert_eq!(result1, String::from("Sao Paulo"));
    }

    #[test]
    fn case_02() {
        let paths = vec![
            vec![String::from("B"), String::from("C")],
            vec![String::from("D"), String::from("B")],
            vec![String::from("C"), String::from("A")],
        ];
        let result1 = Solution::dest_city(paths);
        assert_eq!(result1, String::from("A"));
    }

    #[test]
    fn case_03() {
        let paths = vec![vec![String::from("A"), String::from("Z")]];
        let result1 = Solution::dest_city(paths);
        assert_eq!(result1, String::from("Z"));
    }
}
