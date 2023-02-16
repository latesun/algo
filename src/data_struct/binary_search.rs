fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if target == nums[mid] {
            return Some(mid);
        }

        if target > nums[mid] {
            left = mid + 1;
            continue;
        }
        right = mid - 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = [0, 1, 2, 3, 4, 5];
        assert_eq!(Some(5), binary_search(&nums, 5));

        let nums = [];
        assert!(binary_search(&nums, 5).is_none());
    }
}
