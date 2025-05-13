/*
 * @lc app=leetcode.cn id=123 lang=rust
 * @lcpr version=30204
 *
 * [123] 买卖股票的最佳时机 III
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut pft1 = vec![0; n];
        let mut pft2 = vec![0; n];
        let mut min_price = prices[0];
        for i in 1..n {
            pft1[i] = pft1[i - 1].max(prices[i] - min_price);
            min_price = min_price.min(prices[i]);
        }
        let mut max_price = prices[n - 1];
        for i in (0..n - 1).rev() {
            pft2[i] = pft2[i + 1].max(max_price - prices[i]);
            max_price = max_price.max(prices[i]);
        }

        (1..n)
            .into_iter()
            .map(|d| pft1[d - 1] + pft2[d])
            .max()
            .unwrap_or(0)
            .max(pft1[n - 1])
            .max(pft2[0])
    }
}
// @lc code=end

/*
// @lcpr case=start
// [3,3,5,0,0,3,1,4]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,4,5]\n
// @lcpr case=end

// @lcpr case=start
// [7,6,4,3,1]\n
// @lcpr case=end

// @lcpr case=start
// [1]\n
// @lcpr case=end

 */
