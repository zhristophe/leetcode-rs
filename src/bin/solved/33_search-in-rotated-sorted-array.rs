/*
 * @lc app=leetcode.cn id=33 lang=rust
 * @lcpr version=30204
 *
 * [33] 搜索旋转排序数组
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // 这不是hard吗？
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if (nums[0] > target) ^ (nums[0] > nums[mid]) ^ (target > nums[mid]) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if lo == hi && nums[lo] == target {
            lo as i32
        } else {
            -1
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// [4,5,6,7,0,1,2]\n0\n
// @lcpr case=end

// @lcpr case=start
// [4,5,6,7,0,1,2]\n3\n
// @lcpr case=end

// @lcpr case=start
// [1]\n0\n
// @lcpr case=end

 */
