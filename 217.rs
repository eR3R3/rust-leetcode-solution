use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut record = HashSet::new();
        for &num in nums.iter() {
            if record.contains(&num) {
                return true;
            }
            record.insert(num);
        }
        false
    }
}
