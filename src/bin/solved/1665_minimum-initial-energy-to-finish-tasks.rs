/*
 * @lc app=leetcode.cn id=1665 lang=rust
 * @lcpr version=30204
 *
 * [1665] 完成所有任务的最少初始能量
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::minimum_effort(vec2d![[1, 2], [2, 4], [4, 8]]), 8);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        // 考虑做完某个任务后能量为e，
        // 则做这个任务所需能量为 max(e + act, min)
        let mut tasks = tasks;
        tasks.sort_by_key(|t| t[1] - t[0]);
        // dbg!(&tasks);
        let mut ans = 0;
        for t in tasks.iter() {
            ans = (ans + t[0]).max(t[1]);
        }
        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[1,2],[2,4],[4,8]]\n
// @lcpr case=end

// @lcpr case=start
// [[1,3],[2,4],[10,11],[10,12],[8,9]]\n
// @lcpr case=end

// @lcpr case=start
// [[1,7],[2,8],[3,9],[4,10],[5,11],[6,12]]\n
// @lcpr case=end

 */
