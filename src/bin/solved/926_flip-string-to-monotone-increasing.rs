/*
 * @lc app=leetcode.cn id=926 lang=rust
 * @lcpr version=30204
 *
 * [926] 将字符串翻转到单调递增
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::min_flips_mono_incr("00110".to_string()), 1);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        // 如果最后一位确定为0，答案等于1的数量
        // 如果最后一位确定为1，答案等于前面的答案
        let s = s.as_bytes();
        fn solve(s: &[u8], n_1: i32) -> i32 {
            if s.is_empty() {
                return 0;
            }
            let ans0 = n_1; // 全0
            let n = s.len();
            let ans1 = solve(&s[..n - 1], if s[n - 1] == b'0' { n_1 } else { n_1 - 1 })
                + if s[n - 1] == b'0' { 1 } else { 0 };
            ans0.min(ans1)
        }

        let n_1 = s
            .iter()
            .fold(0, |acc, v| if *v == b'1' { acc + 1 } else { acc });

        solve(s, n_1)
    }
}
// @lc code=end

/*
// @lcpr case=start
// "00110"\n
// @lcpr case=end

// @lcpr case=start
// "010110"\n
// @lcpr case=end

// @lcpr case=start
// "00011000"\n
// @lcpr case=end

 */
