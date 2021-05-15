pub struct Solution {}

impl Solution {
    fn func() -> i32 {
        let a = [1, 2, 3, 0];
        for i in a.iter() {
            println!("{:?}", i);
        }
        for i in &a {
            println!("{:?}", i);
        }
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol() {
        assert_eq!(1, Solution::func());
        assert_eq!(2, Solution::func());
    }
}
