/**
Given a date string in the form Day Month Year, where:

Day is in the set {"1st", "2nd", "3rd", "4th", ..., "30th", "31st"}.
Month is in the set {"Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"}.
Year is in the range [1900, 2100].
Convert the date string to the format YYYY-MM-DD, where:

YYYY denotes the 4 digit year.
MM denotes the 2 digit month.
DD denotes the 2 digit day.


Example 1:

Input: date = "20th Oct 2052"
Output: "2052-10-20"
Example 2:

Input: date = "6th Jun 1933"
Output: "1933-06-06"
Example 3:

Input: date = "26th May 1960"
Output: "1960-05-26"

Constraints:

The given dates are guaranteed to be valid, so no error handling is necessary.

*/

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn reformat_date(date: String) -> String {
        let datemap: HashMap<&str, i32> = [
            ("1st", 1),
            ("2nd", 2),
            ("3rd", 3),
            ("4th", 4),
            ("5th", 5),
            ("6th", 6),
            ("7th", 7),
            ("8th", 8),
            ("9th", 9),
            ("10th", 10),
            ("11th", 11),
            ("12th", 12),
            ("13th", 13),
            ("14th", 14),
            ("15th", 15),
            ("16th", 16),
            ("17th", 17),
            ("18th", 18),
            ("19th", 19),
            ("20th", 20),
            ("21st", 21),
            ("22nd", 22),
            ("23rd", 23),
            ("24th", 24),
            ("25th", 25),
            ("26th", 26),
            ("27th", 27),
            ("28th", 28),
            ("29th", 29),
            ("30th", 30),
            ("31st", 31),
        ]
        .iter()
        .cloned()
        .collect();

        let monthmap: HashMap<&str, i32> = [
            ("Jan", 1),
            ("Feb", 2),
            ("Mar", 3),
            ("Apr", 4),
            ("May", 5),
            ("Jun", 6),
            ("Jul", 7),
            ("Aug", 8),
            ("Sep", 9),
            ("Oct", 10),
            ("Nov", 11),
            ("Dec", 12),
        ]
        .iter()
        .cloned()
        .collect();

        let mut iter = date.split_whitespace();
        let d = iter.next().unwrap();
        let m = iter.next().unwrap();
        let y = iter.next().unwrap();

        return format!(
            "{}-{:02}-{:02}",
            y,
            monthmap.get(m).unwrap(),
            datemap.get(d).unwrap()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol() {
        println!(
            "{:?}",
            Solution::reformat_date(String::from("20th Oct 2052"))
        );
        println!(
            "{:?}",
            Solution::reformat_date(String::from("6th Jun 1933"))
        );
    }
}
