/*
 * @lc app=leetcode.cn id=1497 lang=rust
 * @lcpr version=30204
 *
 * [1497] 检查数组对是否可以被 k 整除
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::can_arrange(vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5),
        true
    );
    assert_eq!(Solution::can_arrange(vec![2, 2, 2, 4], 4), false);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        // 同余系
        let mut cnt = vec![0; k as usize];
        for &x in arr.iter() {
            cnt[(((x % k) + k) % k) as usize] += 1;
        }
        // dbg!(&cnt);
        for i in 1..k as usize {
            if cnt[i] != cnt[k as usize - i] {
                return false;
            }
        }
        if k % 2 == 0 {
            if cnt[k as usize / 2] % 2 != 0 {
                return false;
            }
        }
        true
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,2,3,4,5,10,6,7,8,9]\n5\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,4,5,6]\n7\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,4,5,6]\n10\n
// @lcpr case=end

 */
