/*
 * @lc app=leetcode.cn id=640 lang=rust
 * @lcpr version=30204
 *
 * [640] 求解方程
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::solve_equation("x+5-3+x=6+x-2".to_string()),
        "x=2".to_string()
    );
    assert_eq!(
        Solution::solve_equation("x-x=2x-2".to_string()),
        "x=1".to_string()
    );
    assert_eq!(
        Solution::solve_equation("x+x=2x-2".to_string()),
        "Infinite solutions".to_string()
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let mut k = 0;
        let mut b = 0;
        let eq = equation.chars().collect::<Vec<_>>();
        let n = eq.len();
        let mut i = 0;
        let mut s = 1; // 等号前面或后面
        loop {
            let mut j = i + 1;
            while j < n && matches!(eq[j], '0'..='9' | 'x') {
                j += 1;
            }
            // dbg!((i, j, &eq[i..j]));
            if eq[j - 1] == 'x' {
                k += s * eq[i..j - 1]
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap_or_else(|_| if eq[i] == '-' { -1 } else { 1 });
            } else {
                b += s * eq[i..j].iter().collect::<String>().parse::<i32>().unwrap();
            }
            if j == n {
                break;
            }
            i = j;
            if eq[i] == '=' {
                i += 1;
                s = -1;
            }
        }

        if k == 0 {
            if b == 0 {
                "Infinite solutions".to_string()
            } else {
                "No solution".to_string()
            }
        } else {
            if b % k == 0 {
                format!("x={}", -b / k)
            } else {
                "No solution".to_string()
            }
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// "x=6+x-2"\n
// @lcpr case=end

// @lcpr case=start
// "x=x"\n
// @lcpr case=end

// @lcpr case=start
// "2x=x"\n
// @lcpr case=end

 */
