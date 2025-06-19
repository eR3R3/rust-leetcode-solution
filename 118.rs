impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut dp: Vec<Vec<i32>> = vec![];
        for i in 1..=num_rows {
            dp.push(vec![1; i as usize]);
        }

        for i in 0..num_rows as usize {
            if i == (num_rows - 1) as usize {
                break;
            }
            for j in 0..i as usize {
                dp[i + 1][j + 1] = dp[i][j] + dp[i][j + 1]; 
            }
        }

        dp
    }
}
