/*
 * @lc app=leetcode.cn id=LCP 80 lang=rust
 * @lcpr version=30204
 *
 * [LCP 80] 生物进化录
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::evolutionary_record(vec![-1, 0, 0, 2]),
        "00110".to_string()
    );
    assert_eq!(
        Solution::evolutionary_record(vec![-1, 0, 1, 1, 3, 0, 1, 1, 6, 0, 9, 6, 7, 4, 12, 1, 3, 5]),
        "00001101100011100101101011001100".to_string()
    )
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn evolutionary_record(parents: Vec<i32>) -> String {
        // 首先，开头一定是遍历最深的路径(因为0最多)
        // 然后每步都应该尽量更快地开始走0
        type Tree = std::collections::HashMap<i32, Vec<i32>>;
        let mut tr = Tree::new();
        for (i, &p) in parents.iter().enumerate().skip(1) {
            tr.entry(p).or_insert(vec![]).push(i as i32); // 后一个代表深度
        }
        // 返回的i32是最浅子树的深度，用于去除额外的1
        fn solve(root: i32, tr: &Tree) -> (String, i32) {
            let Some(children) = tr.get(&root) else {
                return (String::from(""), 0);
            };
            let mut children = children
                .iter()
                .map(|c| {
                    let ret = solve(*c, tr);
                    (format!("0{}1", ret.0), ret.1)
                })
                .collect::<Vec<_>>();
            children.sort(); // 遍历结果一样的depth也一定一样
            let mut ans = String::new();
            let mut depth = 0;
            for i in 0..children.len() {
                let c = &children[i];
                ans.push_str(&c.0);
                if i == children.len() - 1 {
                    depth = c.1;
                }
            }

            (ans, depth + 1)
        }
        let (mut ans, depth) = solve(0, &tr);
        for _ in 0..depth {
            ans.pop();
        }
        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [-1,0,0,2]\n
// @lcpr case=end

// @lcpr case=start
// [-1,0,0,1,2,2]\n
// @lcpr case=end

 */
