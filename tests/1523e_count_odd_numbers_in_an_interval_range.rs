/**
Given two non-negative integers low and high. Return the count of odd numbers between low and high (inclusive).

 

Example 1:

Input: low = 3, high = 7
Output: 3
Explanation: The odd numbers between 3 and 7 are [3,5,7].
Example 2:

Input: low = 8, high = 10
Output: 1
Explanation: The odd numbers between 8 and 10 are [9].
 

Constraints:

0 <= low <= high <= 10^9

*/
pub struct Solution {}

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let mut ret = 0;

        if low % 2 ==1 || high %2 == 1 {
            ret = 1;
        }
        ret + (high-low)/2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol() {
        assert_eq!(3, Solution::count_odds(3, 7));
        assert_eq!(1, Solution::count_odds(1, 2));
        assert_eq!(1, Solution::count_odds(2, 3));
        assert_eq!(1, Solution::count_odds(8, 10));
    }
}
