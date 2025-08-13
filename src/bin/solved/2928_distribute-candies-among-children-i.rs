/*
 * @lc app=leetcode.cn id=2928 lang=rust
 * @lcpr version=30204
 *
 * [2928] 给小朋友们分糖果 I
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::distribute_candies(5, 2), 3);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        // 枚举
        let mut ans = 0;
        for a in 0..=limit {
            for b in 0..=limit {
                let c = n - a - b;
                if c >= 0 && c <= limit {
                    ans += 1;
                }
            }
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// 5\n2\n
// @lcpr case=end

// @lcpr case=start
// 3\n3\n
// @lcpr case=end

 */
