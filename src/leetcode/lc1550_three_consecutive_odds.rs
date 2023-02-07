struct Solution {}

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count = 0;
        for v in arr {
            if v % 2 == 0 {
                count = 0;
                continue;
            }

            if count == 2 {
                return true;
            }
            count += 1;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_consecutive_odds() {
        assert!(Solution::three_consecutive_odds(vec![
            1, 2, 34, 3, 4, 5, 7, 23, 12
        ]));
        assert!(!Solution::three_consecutive_odds(vec![2, 6, 4, 1]));
        assert!(!Solution::three_consecutive_odds(vec![1, 2, 1, 1]));
    }
}
