/*
 * @lc app=leetcode.cn id=268 lang=rust
 * @lcpr version=30204
 *
 * [268] 丢失的数字
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut seen = vec![false; nums.len() + 1];
        for num in nums {
            seen[num as usize] = true;
        }
        for i in 0..seen.len() {
            if !seen[i] {
                return i as i32;
            }
        }
        0
    }
}
// @lc code=end

/*
// @lcpr case=start
// [3,0,1]\n
// @lcpr case=end

// @lcpr case=start
// [0,1]\n
// @lcpr case=end

// @lcpr case=start
// [9,6,4,2,3,5,7,0,1]\n
// @lcpr case=end

 */
