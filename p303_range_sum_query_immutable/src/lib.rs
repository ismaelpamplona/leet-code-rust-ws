struct NumArray {
    nums: Vec<i32>,
    prefix_sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix_sum = vec![nums[0]];
        for i in 1..nums.len() {
            prefix_sum.push(nums[i] + prefix_sum[i - 1]);
        }
        Self { nums, prefix_sum }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 {
            return self.prefix_sum[right as usize];
        } else {
            return self.prefix_sum[right as usize] - self.prefix_sum[left as usize - 1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![-2, 0, 3, -5, 2, -1];
        let obj = NumArray::new(nums);
        let result1 = obj.sum_range(0, 2);
        assert_eq!(result1, 1);
        let result2 = obj.sum_range(2, 5);
        assert_eq!(result2, -1);
        let result3 = obj.sum_range(0, 5);
        assert_eq!(result3, -3);
    }
}
