/*
 * @lc app=leetcode.cn id=2717 lang=rust
 * @lcpr version=30204
 *
 * [2717] 半有序排列
 */

// @lcpr-template-start

// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let find = |x: i32| -> usize {
            for (i, &num) in nums.iter().enumerate() {
                if num == x {
                    return i;
                }
            }
            unreachable!()
        };
        let i = find(1);
        let j = find(n as i32);
        (if i < j {
            i + n - 1 - j
        } else {
            let ans = i - j;
            let i = j;
            let j = i + 1;
            ans + i + n - 1 - j
        }) as i32
    }
}
// @lc code=end
struct Solution;
fn main() {
    assert_eq!(Solution::semi_ordered_permutation(vec![2, 1, 4, 3]), 2);
    assert_eq!(Solution::semi_ordered_permutation(vec![2, 4, 1, 3]), 3);
    assert_eq!(Solution::semi_ordered_permutation(vec![1, 3, 4, 2, 5]), 0);
}
/*
// @lcpr case=start
// [2,1,4,3]\n
// @lcpr case=end

// @lcpr case=start
// [2,4,1,3]\n
// @lcpr case=end

// @lcpr case=start
// [1,3,4,2,5]\n
// @lcpr case=end

 */
