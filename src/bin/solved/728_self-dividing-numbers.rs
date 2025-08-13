/*
 * @lc app=leetcode.cn id=728 lang=rust
 * @lcpr version=30204
 *
 * [728] 自除数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::self_dividing_numbers(1, 22),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        // 枚！
        let mut ans = vec![];
        for i in left..=right {
            let mut x = i;
            let mut ok = true;
            while x > 0 {
                let d = x % 10;
                if d == 0 || i % d != 0 {
                    ok = false;
                    break;
                }
                x /= 10;
            }
            if ok {
                ans.push(i);
            }
        }
        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// 1\n22\n
// @lcpr case=end

// @lcpr case=start
// 47\n85\n
// @lcpr case=end

 */
