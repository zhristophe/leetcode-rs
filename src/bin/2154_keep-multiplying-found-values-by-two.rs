/*
 * @lc app=leetcode.cn id=2154 lang=rust
 * @lcpr version=30204
 *
 * [2154] 将找到的值乘以 2
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::find_final_value(vec![5, 3, 6, 1, 12], 3), 24);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut nums = nums;
        let mut original = original;
        nums.sort_unstable();
        let mut i = 0;
        loop {
            if let Ok(idx) = nums[i..].binary_search(&original) {
                original *= 2;
                i = idx + 1;
            } else {
                break original;
            }
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// [5,3,6,1,12]\n3\n
// @lcpr case=end

// @lcpr case=start
// [2,7,9]\n4\n
// @lcpr case=end

 */
