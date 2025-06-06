/*
 * @lc app=leetcode.cn id=2966 lang=rust
 * @lcpr version=30204
 *
 * [2966] 划分数组并满足最大差限制
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::divide_array(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2),
        vec2d![[1, 1, 3], [3, 4, 5], [7, 8, 9]]
    )
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len() / 3;
        let mut ans = vec![vec![0; 3]; n];
        for i in 0..n {
            ans[i][0] = nums[i * 3];
            ans[i][1] = nums[i * 3 + 1];
            ans[i][2] = nums[i * 3 + 2];
            if ans[i][2] - ans[i][0] > k {
                return vec![];
            }
        }
        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,3,4,8,7,9,3,5,1]\n2\n
// @lcpr case=end

// @lcpr case=start
// [2,4,2,2,5,2]\n2\n
// @lcpr case=end

// @lcpr case=start
// [4,2,9,8,2,12,7,12,10,5,8,5,5,7,9,2,5,11]\n14\n
// @lcpr case=end

 */
