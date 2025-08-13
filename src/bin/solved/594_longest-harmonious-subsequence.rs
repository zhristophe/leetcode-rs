/*
 * @lc app=leetcode.cn id=594 lang=rust
 * @lcpr version=30204
 *
 * [594] 最长和谐子序列
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        // 将差为1的数的数量相加
        let mut cnts = std::collections::HashMap::new();
        for &num in nums.iter() {
            *cnts.entry(num).or_insert(0) += 1;
        }
        let mut ns = cnts.keys().cloned().collect::<Vec<_>>();
        ns.sort_unstable();
        let mut ans = 0;
        for i in 1..ns.len() {
            if ns[i] - ns[i - 1] == 1 {
                ans = ans.max(cnts[&ns[i]] + cnts[&ns[i - 1]]);
            }
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,3,2,2,5,2,3,7]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,4]\n
// @lcpr case=end

// @lcpr case=start
// [1,1,1,1]\n
// @lcpr case=end

 */
