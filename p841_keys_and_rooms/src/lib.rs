use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn can_visit_all_rooms_rec(rooms: Vec<Vec<i32>>) -> bool {
        fn dfs(room: usize, seen: &mut HashSet<i32>, rooms: &Vec<Vec<i32>>) {
            for key in &rooms[room] {
                if !seen.contains(&key) {
                    seen.insert(*key);
                    dfs(*key as usize, seen, rooms);
                }
            }
        }
        let mut seen = HashSet::from([0]);
        dfs(0, &mut seen, &rooms);
        seen.len() == rooms.len()
    }

    pub fn can_visit_all_rooms_it(rooms: Vec<Vec<i32>>) -> bool {
        let mut seen = HashSet::from([0]);
        let mut stack = vec![0];
        while let Some(room) = stack.pop() {
            for key in &rooms[room] {
                if !seen.contains(key) {
                    seen.insert(*key);
                    stack.push(*key as usize);
                }
            }
        }
        rooms.len() == seen.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let rooms = vec![vec![1], vec![2], vec![3], vec![]];
        let result1 = Solution::can_visit_all_rooms_rec(rooms.clone());
        // let result2 = Solution::can_visit_all_rooms_it(rooms.clone());
        assert_eq!(result1, true);
        // assert_eq!(result2, true);
    }

    #[test]
    fn case_02() {
        let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
        let result1 = Solution::can_visit_all_rooms_rec(rooms.clone());
        // let result2 = Solution::can_visit_all_rooms_it(rooms.clone());
        assert_eq!(result1, false);
        // assert_eq!(result2, false);
    }

    #[test]
    fn case_03() {
        let rooms = vec![
            vec![4],
            vec![3],
            vec![],
            vec![2, 5, 7],
            vec![1],
            vec![],
            vec![8, 9],
            vec![],
            vec![],
            vec![6],
        ];
        let result1 = Solution::can_visit_all_rooms_rec(rooms.clone());
        // let result2 = Solution::can_visit_all_rooms_it(rooms.clone());
        assert_eq!(result1, false);
        // assert_eq!(result2, false);
    }
}
