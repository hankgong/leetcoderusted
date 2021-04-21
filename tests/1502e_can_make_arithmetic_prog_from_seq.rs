/**
Given an array of numbers arr. A sequence of numbers is called an arithmetic progression if the difference between any two consecutive elements is the same.

Return true if the array can be rearranged to form an arithmetic progression, otherwise, return false.



Example 1:

Input: arr = [3,5,1]
Output: true
Explanation: We can reorder the elements as [1,3,5] or [5,3,1] with differences 2 and -2 respectively, between each consecutive elements.
Example 2:

Input: arr = [1,2,4]
Output: false
Explanation: There is no way to reorder the elements to obtain an arithmetic progression.


Constraints:

2 <= arr.length <= 1000
-10^6 <= arr[i] <= 10^6

*/
pub struct Solution {}

impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        // sort the vector
        let mut sorted = arr.clone();
        sorted.sort();

        let delta = sorted[1] - sorted[0];

        for i in 1..sorted.len() {
            if sorted[i] != sorted[i - 1] + delta {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol() {
        assert_eq!(
            true,
            Solution::can_make_arithmetic_progression(vec![1, 5, 3])
        );
        assert_eq!(
            false,
            Solution::can_make_arithmetic_progression(vec![1, 2, 4])
        );
    }
}
