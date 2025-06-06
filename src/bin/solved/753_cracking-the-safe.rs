/*
 * @lc app=leetcode.cn id=753 lang=rust
 * @lcpr version=30204
 *
 * [753] 破解保险箱
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::crack_safe(1, 2), "01");
    assert_eq!(Solution::crack_safe(2, 2), "01100");
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        // 七桥问题
        if n == 1 {
            return (0..k as u8).map(|i| (i + b'0') as char).collect();
        }
        fn dfs(node: i32, n: i32, k: i32, ans: &mut String, visited: &mut HashSet<i32>) {
            for x in 0..k {
                let next = node * k + x;
                if !visited.contains(&next) {
                    visited.insert(next);
                    dfs(next % k.pow(n as u32 - 1), n, k, ans, visited);
                    ans.push((x as u8 + b'0') as char);
                }
            }
        }
        let mut ans = String::new();
        dfs(0, n, k, &mut ans, &mut HashSet::new());
        ans += "0".repeat(n as usize - 1).as_str();

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// 1\n2\n
// @lcpr case=end

// @lcpr case=start
// 2\n2\n
// @lcpr case=end

 */
