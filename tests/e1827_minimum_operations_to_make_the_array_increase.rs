pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut t = nums[0];
        for i in 1..nums.len() {
            if nums[i] <= t {
                // println!("{:?}", t);
                t += 1;
                res += t-nums[i];
            } else {
                t = nums[i];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol() {
        assert_eq!(3, Solution::min_operations(vec![1, 1, 1]));
        assert_eq!(14, Solution::min_operations(vec![1, 5, 2, 4, 1]));
        assert_eq!(0, Solution::min_operations(vec![8]));
    }
}
