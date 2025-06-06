/*
 * @lc app=leetcode.cn id=2310 lang=rust
 * @lcpr version=30204
 *
 * [2310] 个位数字为 K 的整数之和
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::minimum_numbers(58, 9), 2);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn minimum_numbers(num: i32, k: i32) -> i32 {
        // 根据个位数计算最小需要的数字个数，然后检查是否超过上限
        if num == 0 {
            return 0;
        }
        if k == 0 {
            return if num % 10 == 0 { 1 } else { -1 };
        }
        for i in 1.. {
            if k * i > num {
                return -1;
            }
            if (k * i) % 10 == num % 10 {
                return i;
            }
        }
        unreachable!()
    }
}
// @lc code=end

/*
// @lcpr case=start
// 58\n9\n
// @lcpr case=end

// @lcpr case=start
// 37\n2\n
// @lcpr case=end

// @lcpr case=start
// 0\n7\n
// @lcpr case=end

 */
