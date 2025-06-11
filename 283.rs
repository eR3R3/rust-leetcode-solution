impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let num_zeros = nums.iter().filter(|&&x| x == 0).count();
        nums.retain(|&x| x != 0);
        nums.extend(vec![0; num_zeros as usize]);
    }
}
