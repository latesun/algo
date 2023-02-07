use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());

        for (idx, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                Some(sub_idx) => return vec![*sub_idx as i32, idx as i32],
                None => {
                    map.insert(num, idx);
                }
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
