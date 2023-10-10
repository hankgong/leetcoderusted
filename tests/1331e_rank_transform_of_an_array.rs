/**
Given an array of integers arr, replace each element with its rank.

The rank represents how large the element is. The rank has the following rules:

Rank is an integer starting from 1.
The larger the element, the larger the rank. If two elements are equal, their rank must be the same.
Rank should be as small as possible.


Example 1:

Input: arr = [40,10,20,30]
Output: [4,1,2,3]
Explanation: 40 is the largest element. 10 is the smallest. 20 is the second smallest. 30 is the third smallest.
Example 2:

Input: arr = [100,100,100]
Output: [1,1,1]
Explanation: Same elements share the same rank.
Example 3:

Input: arr = [37,12,28,9,100,56,80,5,12]
Output: [5,3,4,2,8,6,7,1,3]


Constraints:

0 <= arr.length <= 105
-109 <= arr[i] <= 109

*/
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        if arr.len() > 0 {
            let mut sorted = arr.clone();
            sorted.sort();
            let mut hmap = HashMap::new();

            let mut idx = 1;
            let mut cur = sorted[0];
            for &i in sorted.iter() {
                println!("{:?}", &i);

                if i > cur {
                    idx += 1;
                    cur = i;
                }
                hmap.entry(i).or_insert(idx);
            }

            let mut res: Vec<i32> = Vec::new();

            for i in arr.iter() {
                res.push(*hmap.get(i).unwrap());
            }

            res
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol() {
        println!("{:?}", Solution::array_rank_transform(vec![1, 5, 10, 4, 3]));
        println!(
            "{:?}",
            Solution::array_rank_transform(vec![100, 100, 100, 200])
        );
        println!(
            "{:?}",
            Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12])
        );
        println!("{:?}", Solution::array_rank_transform(vec![]));
    }
}
