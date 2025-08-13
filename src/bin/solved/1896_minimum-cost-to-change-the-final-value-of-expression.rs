/*
 * @lc app=leetcode.cn id=1896 lang=rust
 * @lcpr version=30204
 *
 * [1896] 反转表达式值的最少操作次数
 */

// @lcpr-template-start
struct Solution;
fn main() {}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn min_operations_to_flip(expression: String) -> i32 {
        enum Expr {
            Bool(bool),
            Op(bool, Vec<Box<Expr>>),
        }
        fn eval(expr: &Expr) -> bool {
            match expr {
                Expr::Bool(b) => *b,
                Expr::Op(op, children) => {
                    if *op {
                        children.iter().fold(true, |acc, e| acc & eval(e))
                    } else {
                        children.iter().fold(true, |acc, e| acc | eval(e))
                    }
                }
            }
        }
        /// 题目保证有效
        fn parse(mut s: &[u8]) -> (Expr, &[u8]) {
            let mut stack = vec![];
            while s.is_empty() == false {
                match s[0] {
                    b'(' => {
                        let ret = parse(&s[1..]);
                        stack.push(ret.0);
                        s = ret.1;
                    }
                    b'1' => stack.push(Expr::Bool(true)),
                    b'0' => stack.push(Expr::Bool(false)),
                    b'&' => stack.push(Expr::Op(true, vec![])),
                    b'|' => stack.push(Expr::Op(false, vec![])),
                    _ => unreachable!(),
                }
            }
            // 处理&和|的优先级
            let mut ret = vec![]; // 所有或子句
            let mut cur = vec![]; // 当前或子句的原子
            for e in stack.into_iter() {
                match e {
                    Expr::Bool(_) => cur.push(Box::new(e)),
                    Expr::Op(false, _) => (),
                    Expr::Op(true, _) => ret.push(Expr::Op(false, std::mem::take(&mut cur))),
                }
            }

            let ret = if ret.len() > 1 {
                Expr::Op(true, ret)
            } else {
                ret.pop().unwrap()
            };
            (ret,)
        }
        0
    }
}
// @lc code=end

/*
// @lcpr case=start
// "1&(0|1)"\n
// @lcpr case=end

// @lcpr case=start
// "(0&0)&(0&0&0)"\n
// @lcpr case=end

// @lcpr case=start
// "(0|(1|0&1))"\n
// @lcpr case=end

 */
