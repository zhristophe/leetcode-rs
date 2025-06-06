/*
 * @lc app=leetcode.cn id=1317 lang=rust
 * @lcpr version=30204
 *
 * [1317] 将整数转换为两个无零整数的和
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::get_no_zero_integers(2), vec![1, 1])
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for i in 1..n {
            if i.to_string().contains('0') || (n - i).to_string().contains('0') {
                continue;
            }
            return vec![i, n - i];
        }
        unreachable!()
    }
}
// @lc code=end

/*
// @lcpr case=start
// 2\n
// @lcpr case=end

// @lcpr case=start
// 11\n
// @lcpr case=end

// @lcpr case=start
// 10000\n
// @lcpr case=end

// @lcpr case=start
// 69\n
// @lcpr case=end

// @lcpr case=start
// 1010\n
// @lcpr case=end

 */
