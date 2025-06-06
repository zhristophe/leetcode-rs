/*
 * @lc app=leetcode.cn id=75 lang=rust
 * @lcpr version=30204
 *
 * [75] 颜色分类
 */

// @lcpr-template-start
struct Solution;
fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // 无厘头题目
        let mut cnt = [0; 3];
        for n in &*nums {
            cnt[*n as usize] += 1;
        }
        for i in 0..cnt[0] {
            nums[i] = 0;
        }
        for i in cnt[0]..(cnt[0] + cnt[1]) {
            nums[i] = 1;
        }
        for k in (cnt[0] + cnt[1])..nums.len() {
            nums[k] = 2;
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// [2,0,2,1,1,0]\n
// @lcpr case=end

// @lcpr case=start
// [2,0,1]\n
// @lcpr case=end

 */
