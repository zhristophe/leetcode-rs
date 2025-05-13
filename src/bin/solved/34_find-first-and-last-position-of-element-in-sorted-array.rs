/*
 * @lc app=leetcode.cn id=34 lang=rust
 * @lcpr version=30204
 *
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 */

// @lcpr-template-start

// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        let left = l;
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let mid = (l + r + 1) / 2;
            if nums[mid] > target {
                r = mid - 1;
            } else {
                l = mid;
            }
        }
        let right = l;
        if nums[left] == target && nums[right] == target {
            vec![left as i32, right as i32]
        } else {
            vec![-1, -1]
        }
    }
}
// @lc code=end
struct Solution;
fn main() {
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
        vec![3, 4]
    );
}
/*
// @lcpr case=start
// [5,7,7,8,8,10]\n8\n
// @lcpr case=end

// @lcpr case=start
// [5,7,7,8,8,10]\n6\n
// @lcpr case=end

// @lcpr case=start
// []\n0\n
// @lcpr case=end

 */
