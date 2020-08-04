/**
Given an array of integers nums.

A pair (i,j) is called good if nums[i] == nums[j] and i < j.

Return the number of good pairs.

 

Example 1:

Input: nums = [1,2,3,1,1,3]
Output: 4
Explanation: There are 4 good pairs (0,3), (0,4), (3,4), (2,5) 0-indexed.
Example 2:

Input: nums = [1,1,1,1]
Output: 6
Explanation: Each pair in the array are good.
Example 3:

Input: nums = [1,2,3]
Output: 0
 

Constraints:

1 <= nums.length <= 100
1 <= nums[i] <= 100

*/
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut cmap = HashMap::<i32, i32>::new();

        let mut idx = 1;
        for &i in nums.iter(){
            let entry = cmap.entry(i).or_insert(0);
            *entry += 1;
        }
        // println!("{:?}", cmap);
    
        let mut ret = 0;
        for (k, v) in cmap {
            ret += v*(v-1)/2;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol() {
        assert_eq!(4, Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]));
    }
}
