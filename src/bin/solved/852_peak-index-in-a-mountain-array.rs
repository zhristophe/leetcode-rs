/*
 * @lc app=leetcode.cn id=852 lang=rust
 * @lcpr version=30204
 *
 * [852] 山脉数组的峰顶索引
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        // 二分变种
        let (mut lo, mut hi) = (1, arr.len() - 2); // 题目保证是山峰
        while lo < hi {
            let mid = (lo + hi) / 2;
            if arr[mid - 1] > arr[mid] {
                hi = mid - 1;
            } else if arr[mid + 1] > arr[mid] {
                lo = mid + 1;
            } else {
                return mid as i32;
            }
        }

        lo as i32
    }
}
// @lc code=end

/*
// @lcpr case=start
// [0,1,0]\n
// @lcpr case=end

// @lcpr case=start
// [0,2,1,0]\n
// @lcpr case=end

// @lcpr case=start
// [0,10,5,2]\n
// @lcpr case=end

 */
