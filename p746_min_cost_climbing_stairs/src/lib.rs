use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs_bup(cost: Vec<i32>) -> i32 {
        let mut min_cost = vec![0; cost.len() + 1];
        for i in 2..cost.len() + 1 {
            let one_step = min_cost[i - 1] + cost[i - 1];
            let two_steps = min_cost[i - 2] + cost[i - 2];
            min_cost[i] = one_step.min(two_steps);
        }
        min_cost.last().unwrap().to_owned()
    }

    fn min_cost(i: usize, cost: &Vec<i32>, memo: &mut HashMap<usize, i32>) -> i32 {
        if i <= 1 {
            return 0;
        }

        if let Some(val) = memo.get(&i) {
            return *val;
        }
        let one_step = cost[i - 1] + Self::min_cost(i - 1, cost, memo);
        let two_steps = cost[i - 2] + Self::min_cost(i - 2, cost, memo);
        let min = one_step.min(two_steps);
        memo.insert(i, min);
        min
    }

    pub fn min_cost_climbing_stairs_tdown(cost: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();
        Self::min_cost(cost.len(), &cost, &mut memo)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let cost = vec![10, 15, 20];
        let result1 = Solution::min_cost_climbing_stairs_bup(cost.clone());
        let result2 = Solution::min_cost_climbing_stairs_tdown(cost.clone());
        assert_eq!(result1, 15);
        assert_eq!(result2, 15);
    }

    #[test]
    fn case_02() {
        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let result1 = Solution::min_cost_climbing_stairs_bup(cost.clone());
        let result2 = Solution::min_cost_climbing_stairs_tdown(cost.clone());
        assert_eq!(result1, 6);
        assert_eq!(result2, 6);
    }
}
