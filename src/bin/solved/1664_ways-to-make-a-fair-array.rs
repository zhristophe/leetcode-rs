/*
 * @lc app=leetcode.cn id=1664 lang=rust
 * @lcpr version=30204
 *
 * [1664] 生成平衡数组的方案数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::ways_to_make_fair(vec![2, 1, 6, 4]), 1);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        // 预计算奇偶数的和，然后枚举，O(n)
        let n = nums.len();
        if n == 1 {
            return 1;
        }
        let mut sums = vec![0; n];
        sums[0] = nums[0];
        sums[1] = nums[1];
        for i in 2..n {
            sums[i] = sums[i - 2] + nums[i];
        }
        let tot = sums[n - 1] + sums[n - 2];
        let mut ans = 0;
        for i in 0..n {
            // 假设删除i
            let s0 = if i > 0 {
                sums[(i - 1) & !1] // i前的偶数和
            } else {
                0
            };
            let mut s1 = if n % 2 == 0 { sums[n - 1] } else { sums[n - 2] };
            s1 -= if i > 0 {
                sums[(i - 1) | 1] // i和之前的奇数和
            } else {
                0
            };
            if (s0 + s1) * 2 == tot - nums[i] {
                ans += 1;
            }
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [2,1,6,4]\n
// @lcpr case=end

// @lcpr case=start
// [1,1,1]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3]\n
// @lcpr case=end

 */
