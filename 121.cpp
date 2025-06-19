class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int min = 10000;
        int max_profit = 0;

        for (int price: prices) {
            if (price <= min) {
                min = price;
            } else {
                max_profit = max(max_profit, price - min);
            }
        }
        
        return max_profit;
    }
};
