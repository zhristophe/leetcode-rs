/*
 * @lc app=leetcode.cn id=132 lang=rust
 * @lcpr version=30204
 *
 * [132] 分割回文串 II
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::min_cut("aab".to_string()), 1);
    assert_eq!(Solution::min_cut("a".to_string()), 0);
    assert_eq!(Solution::min_cut("ab".to_string()), 1);
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        // 先缓存所有回文串
        let s = s.as_bytes();
        let n = s.len();
        let mut p = vec![vec![false; n]; n]; // 判断[i, j]是不是回文串
        for len in 1..=n {
            for i in 0.. {
                let j = i + len - 1;
                if j >= n {
                    break;
                }
                p[i][j] = s[i] == s[j] && (len <= 2 || p[i + 1][j - 1]);
            }
        }
        // 动态规划
        let mut f = vec![i32::MAX; n];
        for y in 0..n {
            if p[0][y] {
                f[y] = 0;
                continue;
            }
            for x in 1..=y {
                if p[x][y] {
                    f[y] = f[y].min(1 + f[x - 1]);
                }
            }
        }

        f[n - 1]
    }
    #[allow(dead_code)]
    pub fn min_cut_slow(s: String) -> i32 {
        // 先缓存所有回文串
        let s = s.as_bytes();
        let n = s.len();
        let mut memo = HashMap::new();
        for i in 0..n {
            let mut x = i;
            let mut y = i;
            while s[x] == s[y] {
                memo.entry(y).or_insert(vec![]).push(x);
                if x == 0 || y == n - 1 {
                    break;
                }
                x -= 1;
                y += 1;
            }
            let mut x = i;
            let mut y = i + 1;
            while y < n && s[x] == s[y] {
                memo.entry(y).or_insert(vec![]).push(x);
                if x == 0 || y == n - 1 {
                    break;
                }
                x -= 1;
                y += 1;
            }
        }
        // dbg!(&memo);
        // 动态规划
        let mut f = vec![i32::MAX; n];
        for y in 0..n {
            for &x in memo.get(&y).unwrap() {
                let t = if x == 0 { 0 } else { 1 + f[x - 1] };
                f[y] = f[y].min(t);
            }
        }

        f[n - 1]
    }
}
// @lc code=end

/*
// @lcpr case=start
// "aab"\n
// @lcpr case=end

// @lcpr case=start
// "a"\n
// @lcpr case=end

// @lcpr case=start
// "ab"\n
// @lcpr case=end

 */
