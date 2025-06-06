/*
 * @lc app=leetcode.cn id=517 lang=rust
 * @lcpr version=30204
 *
 * [517] 超级洗衣机
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::find_min_moves(vec![1, 0, 5]), 3);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        // 考虑任一位置i，要么i以及i一侧向另一侧流动，要么i向单侧移动
        let n = machines.len();
        let sum: i32 = machines.iter().sum();
        if sum == 0 {
            return 0;
        }
        if sum % n as i32 != 0 {
            return -1;
        }
        let avg = sum / n as i32;
        let mut ans = 0;
        let mut sum = 0;
        for i in 0..n {
            sum += machines[i] - avg;
            ans = ans.max(machines[i] - avg).max(sum.abs());
        }
        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,0,5]\n
// @lcpr case=end

// @lcpr case=start
// [0,3,0]\n
// @lcpr case=end

// @lcpr case=start
// [0,2,0]\n
// @lcpr case=end

 */
