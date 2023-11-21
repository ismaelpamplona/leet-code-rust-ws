use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn climb_stairs_bup(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        let mut map = HashMap::new();
        map.insert(1, 1);
        map.insert(2, 2);
        for i in 3..=n {
            let x = map.get(&(i - 1)).unwrap();
            let y = map.get(&(i - 2)).unwrap();
            map.insert(i, x + y);
        }
        *map.get(&n).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let result = Solution::climb_stairs_bup(2);
        assert_eq!(result, 2);
    }

    #[test]
    fn case_02() {
        let result = Solution::climb_stairs_bup(3);
        assert_eq!(result, 3);
    }
}
