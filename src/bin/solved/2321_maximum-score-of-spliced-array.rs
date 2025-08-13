/*
 * @lc app=leetcode.cn id=2321 lang=rust
 * @lcpr version=30204
 *
 * [2321] 拼接数组的最大分数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::maximums_spliced_array(vec![60, 60, 60], vec![10, 90, 10]),
        210
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        // 取差值，计算最大/最小连续和
        // 技巧：交换数组，以复用代码

        fn solve(nums1: &[i32], nums2: &[i32]) -> i32 {
            let mut max_sum = 0;
            let mut f = 0; // 状态压缩

            // zip速度慢
            // for (a, b) in nums1.iter().zip(nums2.iter()) {
            //     f = f.max(0) + b - a;
            //     max_sum = max_sum.max(f);
            // }
            for i in 0..nums1.len() {
                f = f.max(0) + nums2[i] - nums1[i];
                max_sum = max_sum.max(f);
            }
            return nums1.iter().sum::<i32>() + max_sum;
        }

        solve(&nums1, &nums2).max(solve(&nums2, &nums1))
    }
}
// @lc code=end

/*
// @lcpr case=start
// [60,60,60]\n[10,90,10]\n
// @lcpr case=end

// @lcpr case=start
// [20,40,20,70,30]\n[50,20,50,40,20]\n
// @lcpr case=end

// @lcpr case=start
// [7,11,13]\n[1,1,1]\n
// @lcpr case=end

 */
