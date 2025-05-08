use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut window = HashSet::with_capacity(k + 1);
        for (index, &num) in nums.iter().enumerate() {
            if !window.insert(num) {
                return true;
            }

            if index + 1 > k {
                window.remove(&nums[index - k]);
            }
        }

        false
    }
}
