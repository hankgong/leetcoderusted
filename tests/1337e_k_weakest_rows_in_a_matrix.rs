/**
Given a m * n matrix mat of ones (representing soldiers) and zeros (representing civilians), return the indexes of the k weakest rows in the matrix ordered from the weakest to the strongest.

A row i is weaker than row j, if the number of soldiers in row i is less than the number of soldiers in row j, or they have the same number of soldiers but i is less than j. Soldiers are always stand in the frontier of a row, that is, always ones may appear first and then zeros.



Example 1:

Input: mat =
[[1,1,0,0,0],
 [1,1,1,1,0],
 [1,0,0,0,0],
 [1,1,0,0,0],
 [1,1,1,1,1]],
k = 3
Output: [2,0,3]
Explanation:
The number of soldiers for each row is:
row 0 -> 2
row 1 -> 4
row 2 -> 1
row 3 -> 2
row 4 -> 5
Rows ordered from the weakest to the strongest are [2,0,3,1,4]
Example 2:

Input: mat =
[[1,0,0,0],
 [1,1,1,1],
 [1,0,0,0],
 [1,0,0,0]],
k = 2
Output: [0,2]
Explanation:
The number of soldiers for each row is:
row 0 -> 1
row 1 -> 4
row 2 -> 1
row 3 -> 1
Rows ordered from the weakest to the strongest are [0,2,3,1]


Constraints:

m == mat.length
n == mat[i].length
2 <= n, m <= 100
1 <= k <= m
matrix[i][j] is either 0 or 1.

*/
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        println!("{:?}", mat);
        let mut sval = Vec::<i32>::new();
        let mut sidx = Vec::<i32>::new();

        let mut i = 0;
        for v in mat.iter() {
            let s = v.iter().sum::<i32>();

            let mut l = if sval.len() == 0 { 0 } else { sval.len() }; /* location to insert */
            for j in 0..sval.len() {
                if sval[j] > s {
                    l = j;
                    break;
                }
            }
            // println!("{:?}", l);

            /* insert now. If full, remove the last one */
            // if l < k as usize {
            //     sval.insert(l, s);
            //     sidx.insert(l, i);
            // }

            if l < k as usize {
                if sval.len() == k as usize {
                    sval[l] = s;
                    sidx[l] = i;
                } else {
                    sval.insert(l, s);
                    sidx.insert(l, i);
                }
            }

            i += 1;
        }

        sidx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol() {
        println!(
            "{:?}",
            Solution::k_weakest_rows(
                vec![
                    vec![1, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![1, 0, 0, 0],
                    vec![1, 0, 0, 0]
                ],
                2
            )
        );
    }
}
