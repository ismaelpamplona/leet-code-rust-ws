use std::collections::{HashSet, VecDeque};

struct Solution;
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        if !deadends.contains(&String::from("0000")) {
            let mut q = VecDeque::from([(0000 as u16, 0)]);
            let mut visited = HashSet::from([0000 as u16]);
            for seq in deadends.into_iter() {
                visited.insert(seq.parse::<u16>().unwrap());
            }
            fn neighbors(node: u16) -> Vec<u16> {
                let mut result = vec![];
                for i in &[1000, 100, 10, 1] {
                    let num = (node / i) % 10;
                    for &j in &[
                        (node - i * num) + i * ((num + 1) % 10),
                        (node - i * num) + i * ((num + 9) % 10),
                    ] {
                        result.push(j)
                    }
                }
                result
            }
            while let Some((node, steps)) = q.pop_front() {
                if node == target.parse::<u16>().unwrap() {
                    return steps;
                }
                for neighbour in neighbors(node) {
                    if !visited.contains(&neighbour) {
                        visited.insert(neighbour);
                        q.push_back((neighbour, steps + 1));
                    }
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let deadends = vec![
            String::from("0201"),
            String::from("0101"),
            String::from("0102"),
            String::from("1212"),
            String::from("2002"),
        ];
        let target = String::from("0202");
        let result1 = Solution::open_lock(deadends.clone(), target.clone());
        assert_eq!(result1, 6);
    }

    #[test]
    fn case_02() {
        let deadends = vec![String::from("8888")];
        let target = String::from("0009");
        let result1 = Solution::open_lock(deadends.clone(), target.clone());
        assert_eq!(result1, 1);
    }

    #[test]
    fn case_03() {
        let deadends = vec![
            String::from("8887"),
            String::from("8889"),
            String::from("8878"),
            String::from("8898"),
            String::from("8788"),
            String::from("8988"),
            String::from("7888"),
            String::from("9888"),
        ];
        let target = String::from("8888");
        let result1 = Solution::open_lock(deadends.clone(), target.clone());
        assert_eq!(result1, -1);
    }
}
