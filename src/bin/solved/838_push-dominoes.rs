/*
 * @lc app=leetcode.cn id=838 lang=rust
 * @lcpr version=30204
 *
 * [838] 推多米诺
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::push_dominoes("RR.L".to_string()),
        "RR.L".to_string()
    );
    assert_eq!(
        Solution::push_dominoes(".L.R...LR..L..".to_string()),
        "LL.RR.LLRRLL..".to_string()
    );
    assert_eq!(Solution::push_dominoes("LR".to_string()), "LR".to_string());
    assert_eq!(Solution::push_dominoes("RL".to_string()), "RL".to_string());
    assert_eq!(Solution::push_dominoes("LL".to_string()), "LL".to_string());
    assert_eq!(Solution::push_dominoes("RR".to_string()), "RR".to_string());
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        // 找到连续的'.'，然后根据情况更新dominoes
        let mut dominoes = dominoes.chars().collect::<Vec<_>>();
        macro_rules! next_lr {
            ($pos:expr) => {{
                let mut p = $pos;
                while p < dominoes.len() && dominoes[p] == '.' {
                    p += 1;
                }
                p
            }};
        }
        let mut i = next_lr!(0);
        if i < dominoes.len() && dominoes[i] == 'L' {
            for i in 0..i {
                dominoes[i] = 'L';
            }
        }
        while i < dominoes.len() {
            let j = next_lr!(i + 1);
            if j >= dominoes.len() {
                if dominoes[i] == 'R' {
                    for i in i..dominoes.len() {
                        dominoes[i] = 'R';
                    }
                }
                break;
            }
            match (dominoes[i], dominoes[j]) {
                ('L', 'L') | ('R', 'R') => {
                    for idx in i + 1..j {
                        dominoes[idx] = dominoes[i];
                    }
                }
                ('L', 'R') => {}
                ('R', 'L') => {
                    let (mut i, mut j) = (i + 1, j - 1);
                    while i < j {
                        dominoes[i] = 'R';
                        dominoes[j] = 'L';
                        i += 1;
                        j -= 1;
                    }
                }
                _ => unreachable!(),
            }
            i = j;
        }

        dominoes.iter().collect()
    }
}
// @lc code=end

/*
// @lcpr case=start
// "RR.L"\n
// @lcpr case=end

// @lcpr case=start
// ".L.R...LR..L.."\n
// @lcpr case=end

 */
