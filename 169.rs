use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for &each in nums.iter() {
            *map.entry(each).or_insert(0) += 1;
        }

        let len = nums.len() as i32;
        let mut result = 0;
        
        map.into_iter().for_each(|(key, value)| {
            if value > len / 2 {
                result = key;
            }
        });

        result
    }
}
