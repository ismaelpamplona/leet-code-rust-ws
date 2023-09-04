#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        if dist.len() > hour.ceil() as usize {
            return -1;
        }
        let check = |s: f64| -> bool {
            let mut t: f64 = 0.0;
            for &d in dist.iter() {
                t = t.ceil();
                t += d as f64 / s;
            }
            t <= hour
        };
        let (mut left, mut right) = (1, 10_i32.pow(7));
        while left < right {
            let speed = left + (right - left) / 2;
            match check(speed as f64) {
                false => left = speed + 1,
                true => right = speed,
            }
        }

        left
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let dist = vec![1, 3, 2];
        let hour: f64 = 6.0;
        let result1 = Solution::min_speed_on_time(dist, hour);
        assert_eq!(result1, 1);
    }

    #[test]
    fn case_02() {
        let dist = vec![1, 3, 2];
        let hour: f64 = 2.7;
        let result1 = Solution::min_speed_on_time(dist, hour);
        assert_eq!(result1, 3);
    }

    #[test]
    fn case_03() {
        let dist = vec![1, 3, 2];
        let hour: f64 = 1.9;
        let result1 = Solution::min_speed_on_time(dist, hour);
        assert_eq!(result1, -1);
    }
}
