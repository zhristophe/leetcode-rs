/*
 * @lc app=leetcode.cn id=3335 lang=rust
 * @lcpr version=30204
 *
 * [3335] 字符串转换后的长度 I
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::length_after_transformations("abcyy".to_string(), 2),
        7
    );
        assert_eq!(
        Solution::length_after_transformations("azbk".to_string(), 1),
        5
    );
}
// @lcpr-template-end
// @lc code=start
const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        // dp
        let mut dp = [1; 26];
        for _ in 0..t {
            let mut dp2 = [1; 26];
            for i in 0..25 {
                dp2[i] = dp[i + 1]
            }
            dp2[25] = (dp[0] + dp[1]) % MOD;
            dp = dp2;
        }
        let mut ans = 0;
        for ch in s.chars() {
            ans += dp[ch as usize - 'a' as usize];
            ans %= MOD;
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// "abcyy"\n2\n
// @lcpr case=end

// @lcpr case=start
// "azbk"\n1\n
// @lcpr case=end

 */
