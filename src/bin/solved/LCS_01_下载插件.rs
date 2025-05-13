/*
 * @lc app=leetcode.cn id=LCS 01 lang=rust
 * @lcpr version=30204
 *
 * [LCS 01] 下载插件
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::least_minutes(2), 2);
    assert_eq!(Solution::least_minutes(3), 3);
    assert_eq!(Solution::least_minutes(4), 3);
    assert_eq!(Solution::least_minutes(100000), 18);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn least_minutes(n: i32) -> i32 {
        let mut ans = n;
        for i in 1..20 {
            let speed = 1 << i;
            let new_ans = i + ((n + speed - 1) / speed);
            ans = ans.min(new_ans);
        }
        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// 2`>\n
// @lcpr case=end

// @lcpr case=start
// 4`>\n
// @lcpr case=end

 */
