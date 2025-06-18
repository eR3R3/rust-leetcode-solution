// Source: https://usaco.guide/general/io

#include <bits/stdc++.h>
using namespace std;

const int max_day = 1000;

int main() {
    ifstream read("time.in");

	int n, m, c;
	read >> n >> m >> c;
    
    vector<int> money(n);
    for (int i = 0; i < n; i++) {
        read >> money[i];
    }

    vector<vector<int>> graph(n);
    for (int i = 0; i < m; i++) {
        int a , b;
        read >> a >> b;
        graph[a - 1].push_back(b - 1);
    }
    
    vector<vector<int>> dp(max_day, vector<int>(n, -1));
    dp[0][0] = 0;

    int result = 0;

    for (int i = 0; i < max_day; i++) {
        for (int j = 0; j < n; j++) {
            if (dp[i][j] == -1) {
                continue;
            }

            for (int des: graph[j]) {
                if (i + 1 < max_day) {
                    dp[i + 1][des] = max(dp[i + 1][des], dp[i][j] + money[des]);
                }
            }
        }
        result = max(result, dp[i][0] - c * i * i);
    }
    ofstream("time.out") << result << "\n";
}
