use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut seq: Vec<i32> = vec![];
        let mut toggle = true;
        for row in board.iter().rev() {
            if toggle {
                seq.extend(row.iter());
            } else {
                seq.extend(row.iter().rev());
            }
            toggle = !toggle;
        }
        let mut q = VecDeque::from([0]);
        let mut best = vec![-1; n * n];
        best[0] = 0;
        while let Some(i) = q.pop_front() {
            for j in i + 1..(i + 7).min(n * n) {
                let mut ni = j;

                if seq[ni] != -1 {
                    ni = (seq[ni] - 1) as usize;
                }

                if ni == (n * n) - 1 {
                    return best[i] + 1;
                }

                if best[ni] == -1 {
                    q.push_back(ni);
                    best[ni] = best[i] + 1;
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
        let board = vec![
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 35, -1, -1, 13, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 15, -1, -1, -1, -1],
        ];
        let result1 = Solution::snakes_and_ladders(board);
        assert_eq!(result1, 4);
    }

    #[test]
    fn case_02() {
        let board = vec![vec![-1, -1], vec![-1, 3]];
        let result1 = Solution::snakes_and_ladders(board);

        assert_eq!(result1, 1);
    }
}
