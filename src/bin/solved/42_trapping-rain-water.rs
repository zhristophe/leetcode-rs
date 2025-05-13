/*
 * @lc app=leetcode.cn id=42 lang=rust
 * @lcpr version=30204
 *
 * [42] 接雨水
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    let mut height = vec![10_000];
    for _ in 0..10_000 {
        height.push(0);
    }
    height.push(99_999);
    for _ in 0..10_000 {
        height.push(0);
    }
    height.push(10_000);
    assert_eq!(Solution::trap(height), 10_000 * 20_000);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();

        let mut water = vec![i32::MAX; n];
        let mut h = 0;
        for i in 0..n {
            h = h.max(height[i]);
            water[i] = water[i].min(h);
        }
        h = 0;
        for i in (0..n).rev() {
            h = h.max(height[i]);
            water[i] = water[i].min(h);
        }
        water
            .iter()
            .zip(height.iter())
            .fold(0, |acc, (w, h)| acc + w - h)
    }
}
// @lc code=end

/*
// @lcpr case=start
// [0,1,0,2,1,0,1,3,2,1,2,1]\n
// @lcpr case=end

// @lcpr case=start
// [4,2,0,3,2,5]\n
// @lcpr case=end

 */
