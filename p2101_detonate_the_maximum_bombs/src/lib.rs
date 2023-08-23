use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;
impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut graph = HashMap::new();
        let n = bombs.len();
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                };
                let bomb_a = &bombs[i];
                let bomb_b = &bombs[j];
                let (dx, dy) = (bomb_a[0] - bomb_b[0], bomb_a[1] - bomb_b[1]);
                let distance = dx as i64 * dx as i64 + dy as i64 * dy as i64;
                if distance <= (bombs[i][2] as i64 * bombs[i][2] as i64) {
                    graph.entry(i).or_insert(vec![]).push(j);
                }
            }
        }
        fn bfs(i: usize, graph: &HashMap<usize, Vec<usize>>) -> i32 {
            let mut q = VecDeque::from([i]);
            let mut seen = HashSet::from([i]);
            while let Some(i) = q.pop_front() {
                if let Some(neighbours) = graph.get(&i) {
                    for neighbour in neighbours {
                        if !seen.contains(neighbour) {
                            seen.insert(*neighbour);
                            q.push_back(*neighbour);
                        }
                    }
                }
            }
            seen.len() as i32
        }
        let mut ans = 0;
        for i in 0..n {
            ans = ans.max(bfs(i, &graph));
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let bombs = vec![vec![2, 1, 3], vec![6, 1, 4]];
        let result1 = Solution::maximum_detonation(bombs.clone());
        assert_eq!(result1, 2);
    }

    #[test]
    fn case_02() {
        let bombs = vec![vec![1, 1, 5], vec![10, 10, 5]];
        let result1 = Solution::maximum_detonation(bombs.clone());
        assert_eq!(result1, 1);
    }

    #[test]
    fn case_03() {
        let bombs = vec![
            vec![1, 2, 3],
            vec![2, 3, 1],
            vec![3, 4, 2],
            vec![4, 5, 3],
            vec![5, 6, 4],
        ];
        let result1 = Solution::maximum_detonation(bombs.clone());
        assert_eq!(result1, 5);
    }

    #[test]
    fn case_04() {
        let bombs = vec![
            vec![54, 95, 4],
            vec![99, 46, 3],
            vec![29, 21, 3],
            vec![96, 72, 8],
            vec![49, 43, 3],
            vec![11, 20, 3],
            vec![2, 57, 1],
            vec![69, 51, 7],
            vec![97, 1, 10],
            vec![85, 45, 2],
            vec![38, 47, 1],
            vec![83, 75, 3],
            vec![65, 59, 3],
            vec![33, 4, 1],
            vec![32, 10, 2],
            vec![20, 97, 8],
            vec![35, 37, 3],
        ];
        let result1 = Solution::maximum_detonation(bombs.clone());
        assert_eq!(result1, 1);
    }

    #[test]
    fn case_05() {
        let bombs = vec![
            vec![85024, 58997, 3532],
            vec![65196, 42043, 9739],
            vec![85872, 75029, 3117],
            vec![73014, 91183, 7092],
            vec![29098, 40864, 7624],
            vec![11469, 13607, 4315],
            vec![98722, 69681, 9656],
            vec![75140, 42250, 421],
            vec![92580, 44040, 4779],
            vec![58474, 78273, 1047],
            vec![27683, 4203, 6186],
            vec![10714, 24238, 6243],
            vec![60138, 81791, 3496],
            vec![16227, 92418, 5622],
            vec![60496, 64917, 2463],
            vec![59241, 62074, 885],
            vec![11961, 163, 5815],
            vec![37757, 43214, 3402],
            vec![21094, 98519, 1678],
            vec![49368, 22385, 1431],
            vec![6343, 53798, 159],
            vec![80129, 9282, 5139],
            vec![69565, 32036, 6827],
            vec![59372, 64978, 6575],
            vec![44948, 71199, 7095],
            vec![46390, 91701, 1667],
            vec![37144, 98691, 8128],
            vec![13558, 81505, 4653],
            vec![41234, 48161, 9304],
            vec![14852, 3206, 5369],
        ];
        let result1 = Solution::maximum_detonation(bombs.clone());
        assert_eq!(result1, 3);
    }
}
