/**
Given an integer n. Each number from 1 to n is grouped according to the sum of its digits.

Return how many groups have the largest size.



Example 1:

Input: n = 13
Output: 4
Explanation: There are 9 groups in total, they are grouped according sum of its digits of numbers from 1 to 13:
[1,10], [2,11], [3,12], [4,13], [5], [6], [7], [8], [9]. There are 4 groups with largest size.
Example 2:

Input: n = 2
Output: 2
Explanation: There are 2 groups [1], [2] of size 1.
Example 3:

Input: n = 15
Output: 6
Example 4:

Input: n = 24
Output: 5


Constraints:

1 <= n <= 10^4

*/
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut tmap = HashMap::<i32, i32>::new();
        for i in 1..=n {
            let s = i
                .to_string()
                .chars()
                .map(|d| d.to_digit(10).unwrap() as i32)
                .sum::<i32>();

            *tmap.entry(s).or_insert(0) += 1;
            // println!("{:?}", tmap);
        }

        let mut ret = 0;
        let mut lsize = 0;
        tmap.iter().for_each(|(_, &c)| {
            if c > lsize {
                lsize = c;
                ret = 1;
            } else if c == lsize {
                ret += 1;
            }
        });

        ret
    }

    pub fn count_largest_group1(n: i32) -> i32 {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut max = 0;
        for i in 1..=n {
            let mut sum = 0;
            let mut n = i;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }
            let count = hm.entry(sum).or_default();
            *count += 1;
            max = max.max(*count);
        }
        hm.values().filter(|&&v| v == max).count() as i32
    }

    pub fn digits_sum(i: i32) -> i32 {
        let mut k = i;
        let mut sum = 0;

        while k > 0 {
            sum += k % 10;
            k /= 10;
        }

        sum
    }

    pub fn count_largest_group2(n: i32) -> i32 {
        let mut groups = [0; 37];

        for i in 1..n + 1 {
            let sum = Solution::digits_sum(i);
            groups[sum as usize] += 1;
        }

        let mut largest_size = 0;
        let mut cnt = 0;

        for i in groups.iter() {
            if *i > largest_size {
                cnt = 1;
                largest_size = *i;
            } else if *i == largest_size {
                cnt += 1;
            }
        }

        cnt
    }

    pub fn count_largest_group3(n: i32) -> i32 {
        let mut counts: [i32; 37] = [0; 37];

        (1..=n)
            .map(|x| Solution::digits_sum(x))
            .for_each(|s| counts[s as usize] += 1);

        let m = counts.iter().max().unwrap();
        return counts.iter().filter(|n| *n == m).count() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol() {
        assert_eq!(6, Solution::count_largest_group(15));
        println!("{:?}", 3.max(5));
    }
}
