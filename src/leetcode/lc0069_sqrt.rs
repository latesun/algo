struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut left = 1;
        let mut right = x;
        let mut res = 0;

        while left <= right {
            let mid = (right + left) / 2;
            if mid == x / mid {
                return mid;
            }

            if mid > x / mid {
                right = mid - 1;
            } else {
                left = mid + 1;
                res = mid;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_sqrt() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}
