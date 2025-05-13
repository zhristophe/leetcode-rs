/*
 * @lc app=leetcode.cn id=188 lang=rust
 * @lcpr version=30204
 *
 * [188] 买卖股票的最佳时机 IV
 */

// @lcpr-template-start
struct Solution;
fn main() {
    // assert_eq!(Solution::max_profit(11454, vec![2, 4, 1, 4]), 5);
    // assert_eq!(Solution::max_profit(1, vec![2, 4, 1, 4]), 3);
    // assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
    assert_eq!(Solution::max_profit(2, vec![6, 1, 3, 2, 4, 7]), 7);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let k = k as usize;
        let mut buy = vec![0; k + 1];
        let mut sell = vec![0; k + 1];
        buy[0] = -prices[0];
        sell[0] = 0;
        for i in 1..=k {
            buy[i] = i32::MIN / 2;
            sell[i] = i32::MIN / 2;
        }
        for i in 1..n {
            buy[0] = buy[0].max(sell[0] - prices[i]);
            for j in 1..=k {
                buy[j] = buy[j].max(sell[j] - prices[i]);
                sell[j] = sell[j].max(buy[j - 1] + prices[i]);
            }
        }

        sell.iter().max().unwrap().clone()
    }
}
// @lc code=end

/*
// @lcpr case=start
// 2\n[2,4,1]\n
// @lcpr case=end

// @lcpr case=start
// 2\n[3,2,6,5,0,3]\n
// @lcpr case=end

 */
