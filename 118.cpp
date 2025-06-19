class Solution {
public:
    vector<vector<int>> generate(int num_rows) {
        vector<vector<int>> result;

        result.push_back(std::vector<int>{1});

        for (int i = 1; i < num_rows; i++) {
            vector<int> curr(i + 1, 1);
            for (int j = 0; j <= i - 2; j++) {
                curr[j + 1] = result[i - 1][j] + result[i - 1][j + 1];
            }
            
            result.push_back(curr);
        }

        return result;
    }
};
