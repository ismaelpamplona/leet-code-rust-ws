use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut set: HashSet<(i32, i32)> = HashSet::new();
        set.insert((0, 0));
        let map = HashMap::from([('N', (0, 1)), ('S', (0, -1)), ('E', (1, 0)), ('W', (-1, 0))]);
        let pv: Vec<char> = path.chars().collect();
        let mut last = (0, 0);

        for i in 0..pv.len() {
            if let Some(dir) = map.get(&pv[i]) {
                let hor = dir.0;
                let ver = dir.1;
                last = (last.0 + hor, last.1 + ver);

                if set.contains(&last) {
                    return true;
                }

                set.insert(last);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let path = String::from("NES");
        let result = Solution::is_path_crossing(path);
        assert_eq!(result, false);
    }

    #[test]
    fn case_02() {
        let path = String::from("NESWW");
        let result = Solution::is_path_crossing(path);
        assert_eq!(result, true);
    }

    #[test]
    fn case_03() {
        let path = String::from("SS");
        let result = Solution::is_path_crossing(path);
        assert_eq!(result, false);
    }
}
