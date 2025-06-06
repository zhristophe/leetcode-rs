/*
 * @lc app=leetcode.cn id=LCR 168 lang=rust
 * @lcpr version=30204
 *
 * [LCR 168] 丑数
 */

use std::i32;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::nth_ugly_number(2), 2);
    assert_eq!(Solution::nth_ugly_number(10), 12);
    Solution::nth_ugly_number(1690);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        // 测试一下就知道结果特别大，不应该一个个枚举
        // 但是n很小
        let mut pq = std::collections::BTreeSet::new();
        pq.insert(1);
        let mut n = n;
        loop {
            let x = pq.pop_first().unwrap();
            // dbg!(x);
            if n == 1 {
                return x;
            }
            n -= 1;

            for i in [2, 3, 5] {
                pq.insert(x.checked_mul(i).unwrap_or(i32::MAX));
            }
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// 10\n
// @lcpr case=end

 */
