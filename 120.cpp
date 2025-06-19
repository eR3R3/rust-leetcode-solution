class Solution {
public:
    int minimumTotal(vector<vector<int>>& triangle) {
        int len = triangle.size();
        vector<vector<int>> dp;

        for (int i = 0; i < len; i++) {
            dp.push_back(std::vector(i + 1, 0));
        }
        
        dp[0][0] = triangle[0][0];

        for (int i = 0; i < len - 1; i++) {
            dp[i + 1][0] = dp[i][0] + triangle[i + 1][0];
            dp[i + 1][i + 1] = dp[i][i] + triangle[i + 1][i + 1];
            for (int j = 1; j <= i; j++) {
                dp[i + 1][j] = triangle[i + 1][j] + min(dp[i][j - 1], dp[i][j]);
            }
        }

        int minimum = 100000000;

        for (int i: dp[len - 1]) {
            minimum = min(minimum, i);
        }
        return minimum;
    }
};
