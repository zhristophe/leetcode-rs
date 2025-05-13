/*
 * @lc app=leetcode.cn id=371 lang=rust
 * @lcpr version=30204
 *
 * [371] 两整数之和
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::get_sum(1, 2), 3)
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        // 无需考虑符号位
        let mut and = a & b;
        let mut xor = a ^ b;
        and <<= 1;
        let mut res = xor;
        while and != 0 {
            res ^= and;
            and &= xor;
            and <<= 1;
            xor = res;
        }
        res
    }
}
// @lc code=end

/*
// @lcpr case=start
// 1\n2\n
// @lcpr case=end

// @lcpr case=start
// 2\n3\n
// @lcpr case=end

 */
