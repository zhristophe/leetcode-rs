/*
 * @lc app=leetcode.cn id=2164 lang=rust
 * @lcpr version=30204
 *
 * [2164] 对奇偶下标分别排序
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::sort_even_odd(vec![4, 1, 2, 3]), vec![2, 3, 4, 1])
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        let mut n0 = vec![];
        let mut n1 = vec![];
        for i in 0..nums.len() {
            if i % 2 == 0 {
                n0.push(nums[i]);
            } else {
                n1.push(nums[i]);
            }
        }
        n0.sort();
        n1.sort_by_key(|a| std::cmp::Reverse(*a));
        let mut nums = nums;
        for i in 0..n0.len() {
            nums[i * 2] = n0[i];
        }
        for i in 0..n1.len() {
            nums[i * 2 + 1] = n1[i];
        }

        nums
    }
}
// @lc code=end

/*
// @lcpr case=start
// [4,1,2,3]\n
// @lcpr case=end

// @lcpr case=start
// [2,1]\n
// @lcpr case=end

 */
