/*
 * @lc app=leetcode.cn id=3228 lang=rust
 * @lcpr version=30204
 *
 * [3228] 将 1 移动到末尾的最大操作次数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::max_operations(String::from("1001101")), 4);
    assert_eq!(Solution::max_operations(String::from("11001101")), 6);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn max_operations(s: String) -> i32 {
        // 最后一个1，如果后面没0，去掉不影响结果。
        // 如果有0，等于前面的操作数 + 1 + 前面的1数量
        let s = s.as_bytes();
        let n_1 = s
            .iter()
            .fold(0, |acc, v| if *v == b'1' { acc + 1 } else { acc });
        fn solve(s: &[u8], n_1: i32) -> i32 {
            let n = s.len();
            match s.last() {
                None => 0,
                Some(v) => match *v {
                    b'0' => {
                        let mut i = n - 1;
                        loop {
                            if s[i] == b'1' {
                                break;
                            }
                            if i == 0 {
                                return 0;
                            }
                            i -= 1;
                        }
                        solve(&s[..=i], n_1) + n_1
                    }
                    b'1' => solve(&s[..n - 1], n_1 - 1),
                    _ => unreachable!(),
                },
            }
        }

        solve(s, n_1)
    }
}
// @lc code=end

/*
// @lcpr case=start
// "1001101"\n
// @lcpr case=end

// @lcpr case=start
// "00111"\n
// @lcpr case=end

 */
