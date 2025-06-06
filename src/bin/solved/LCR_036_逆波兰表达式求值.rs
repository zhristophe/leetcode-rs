/*
 * @lc app=leetcode.cn id=LCR 036 lang=rust
 * @lcpr version=30204
 *
 * [LCR 036] 逆波兰表达式求值
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::eval_rpn(vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string()
        ]),
        9
    )
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        for token in tokens {
            let res = match token.as_str() {
                "+" => stack.pop().unwrap() + stack.pop().unwrap(),
                "-" => -stack.pop().unwrap() + stack.pop().unwrap(),
                "*" => stack.pop().unwrap() * stack.pop().unwrap(),
                "/" => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    v2 / v1
                }
                _ => token.parse::<i32>().unwrap(),
            };
            stack.push(res);
        }
        stack[0]
    }
}
// @lc code=end

/*
// @lcpr case=start
// ["2","1","+","3","*"]\n
// @lcpr case=end

// @lcpr case=start
// ["4","13","5","/","+"]\n
// @lcpr case=end

// @lcpr case=start
// ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]\n
// @lcpr case=end

 */
