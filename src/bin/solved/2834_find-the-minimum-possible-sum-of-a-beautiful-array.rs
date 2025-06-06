/*
 * @lc app=leetcode.cn id=2834 lang=rust
 * @lcpr version=30204
 *
 * [2834] 找出美丽数组的最小和
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::minimum_possible_sum(2, 3), 4);
}
// @lcpr-template-end
// @lc code=start
const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
        // [0,tgt/2], [tgt, ]
        let mut ans = 0;
        let i = n.min(target / 2) as i64;
        ans += i * (i + 1) / 2;
        ans %= MOD as i64;
        let n = n as i64 - i;
        if n > 0 {
            ans += (target as i64 * 2 + n - 1) * n / 2;
            ans %= MOD as i64;
        }

        ans as i32
    }
}
// @lc code=end

/*
// @lcpr case=start
// 2\n3\n
// @lcpr case=end

// @lcpr case=start
// 3\n3\n
// @lcpr case=end

// @lcpr case=start
// 1\n1\n
// @lcpr case=end

 */
