/*
 * @lc app=leetcode.cn id=639 lang=rust
 * @lcpr version=30204
 *
 * [639] 解码方法 II
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::num_decodings("0".to_string()), 0);
    assert_eq!(Solution::num_decodings("1".to_string()), 1);
    assert_eq!(Solution::num_decodings("*".to_string()), 9);
    assert_eq!(Solution::num_decodings("1*".to_string()), 18);
    assert_eq!(Solution::num_decodings("2*".to_string()), 15);
    assert_eq!(Solution::num_decodings("3*".to_string()), 9);
    assert_eq!(Solution::num_decodings("**".to_string()), 96);
    assert_eq!(Solution::num_decodings("0**".to_string()), 0);
    assert_eq!(Solution::num_decodings("12".to_string()), 2);
}
// @lcpr-template-end
// @lc code=start
const MOD: i64 = 1_000_000_007;
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        // 动态规划
        // f2表示上两个的解码数，f1表示上一个的解码数
        let chs = s.chars().collect::<Vec<_>>();
        let (mut f2, mut f1): (i64, i64) = (
            1,
            match chs[0] {
                '0' => 0,
                '*' => 9,
                _ => 1,
            },
        );
        for i in 1..chs.len() {
            // dbg!((f2, f1));
            let single = match chs[i] {
                '0' => 0,
                '*' => 9,
                _ => 1,
            };
            let group = match chs[i - 1] {
                '0' => 0,
                '1' => match chs[i] {
                    '*' => 9,
                    _ => 1,
                },
                '2' => match chs[i] {
                    '*' => 6,
                    '0'..='6' => 1,
                    _ => 0,
                },
                '*' => match chs[i] {
                    '*' => 15,
                    '0'..='6' => 2,
                    _ => 1,
                },
                _ => 0,
            };
            let f = (single * f1 + group * f2) % MOD;
            (f2, f1) = (f1, f);
        }

        f1 as i32
    }
}
// @lc code=end

/*
// @lcpr case=start
// "*"\n
// @lcpr case=end

// @lcpr case=start
// "1*"\n
// @lcpr case=end

// @lcpr case=start
// "2*"\n
// @lcpr case=end

 */
