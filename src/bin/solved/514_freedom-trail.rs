/*
 * @lc app=leetcode.cn id=514 lang=rust
 * @lcpr version=30204
 *
 * [514] 自由之路
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::find_rotate_steps("godding".to_string(), "gd".to_string()),
        4
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        // 简单动态规划
        let ring = ring.chars().collect::<Vec<_>>();
        let key = key.chars().collect::<Vec<_>>();
        // 预处理获得char和index的对应关系
        let mut char_index = HashMap::new();
        for (i, c) in ring.iter().enumerate() {
            char_index.entry(*c).or_insert(vec![]).push(i);
        }
        let mut dp = vec![vec![i32::MAX; ring.len()]; key.len()];
        let cal_dis = |i, j| {
            let ring_len = ring.len() as i32;
            let dis = (i as i32 - j as i32).abs();
            dis.min((ring_len - dis) % ring_len)
        };
        for j in &char_index[&key[0]] {
            dp[0][*j] = cal_dis(0, *j) + 1;
        }
        for (i, tgt) in key.iter().enumerate().skip(1) {
            for &j in &char_index[tgt] {
                for &j0 in &char_index[&key[i - 1]] {
                    if dp[i - 1][j0] == i32::MAX {
                        continue;
                    }
                    dp[i][j] = dp[i][j].min(dp[i - 1][j0] + cal_dis(j, j0) + 1);
                }
            }
        }
        // dbg!(&dp);

        dp.last().unwrap().iter().min().unwrap().clone()
    }
}
// @lc code=end

/*
// @lcpr case=start
// "godding"\n"gd"\n
// @lcpr case=end

// @lcpr case=start
// "godding"\n"godding"\n
// @lcpr case=end

 */
