/*
 * @lc app=leetcode.cn id=2896 lang=rust
 * @lcpr version=30204
 *
 * [2896] 执行操作使两个字符串相等
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::min_operations("1100011000".to_string(), "0101001010".to_string(), 2),
        4
    );
    assert_eq!(
        Solution::min_operations("10110".to_string(), "00011".to_string(), 2),
        -1
    );
    assert_eq!(
        Solution::min_operations("1111".to_string(), "0000".to_string(), 10),
        2
    );
    assert_eq!(
        Solution::min_operations("11001011111".to_string(), "01111000110".to_string(), 2),
        4
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn min_operations(s1: String, s2: String, x: i32) -> i32 {
        let s = s1
            .bytes()
            .zip(s2.bytes())
            .map(|(a, b)| (a - b'0') ^ (b - b'0'))
            .collect::<Vec<_>>();
        let ones = s
            .iter()
            .enumerate()
            .filter_map(|(index, x)| if *x == 1 { Some(index) } else { None })
            .collect::<Vec<_>>();
        if ones.len() % 2 == 1 {
            return -1;
        } else if ones.len() == 0 {
            return 0;
        }
        if x == 0 {
            return 0;
        } else if x == 1 {
            return ones.len() as i32 / 2;
        }
        // 按1动归, [i, i + 2 * d - 1]区间
        let mut dp = vec![vec![i32::MAX; ones.len() / 2 + 1]; ones.len()];
        for i in 0..ones.len() - 1 {
            dp[i][1] = x.min((ones[i + 1] - ones[i]) as i32);
        }
        for d in 2..=ones.len() / 2 {
            for i in 0..=ones.len() - 2 * d {
                dp[i][d] = x + dp[i + 1][d - 1];
                for d0 in 1..d {
                    dp[i][d] = dp[i][d].min(dp[i][d0] + dp[i + 2 * d0][d - d0]);
                }
            }
        }

        dp[0][ones.len() / 2]
    }
}
// @lc code=end

/*
// @lcpr case=start
// "1100011000"\n"0101001010"\n2\n
// @lcpr case=end

// @lcpr case=start
// "10110"\n"00011"\n4\n
// @lcpr case=end

 */
