/*
 * @lc app=leetcode.cn id=441 lang=rust
 * @lcpr version=30204
 *
 * [441] 排列硬币
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::arrange_coins(5), 2);
    assert_eq!(Solution::arrange_coins(8), 3);
    assert_eq!(Solution::arrange_coins(1804289383), 60070);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut l = 0;
        let mut r = (2.0 * n as f64).sqrt() as i64 + 1;
        while l < r {
            let mid = l + (r - l + 1) / 2;
            if mid * (mid + 1) <= 2 * n as i64 {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        l as i32
    }
}
// @lc code=end

/*
// @lcpr case=start
// 5\n
// @lcpr case=end

// @lcpr case=start
// 8\n
// @lcpr case=end

 */
