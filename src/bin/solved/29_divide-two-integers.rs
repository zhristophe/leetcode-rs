/*
 * @lc app=leetcode.cn id=29 lang=rust
 * @lcpr version=30204
 *
 * [29] 两数相除
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::divide(10, 3), 3);
    assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
    assert_eq!(Solution::divide(i32::MIN, i32::MIN), 1);
    assert_eq!(Solution::divide(i32::MIN, i32::MAX), -1);
    assert_eq!(Solution::divide(i32::MAX, i32::MIN), 0);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        // 无厘头题目
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        dividend / divisor
    }
}
// @lc code=end

/*
// @lcpr case=start
// 10\n3\n
// @lcpr case=end

// @lcpr case=start
// 7\n-3\n
// @lcpr case=end

 */
