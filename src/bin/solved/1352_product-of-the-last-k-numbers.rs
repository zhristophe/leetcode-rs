/*
 * @lc app=leetcode.cn id=1352 lang=rust
 * @lcpr version=30204
 *
 * [1352] 最后 K 个数的乘积
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
fn main() {
    let mut s = ProductOfNumbers::new();
    let inc = [
        "ProductOfNumbers",
        "add",
        "add",
        "add",
        "add",
        "add",
        "getProduct",
        "getProduct",
        "getProduct",
        "add",
        "getProduct",
    ];
    let args = vec2d![[], [3], [0], [2], [5], [4], [2], [3], [4], [8], [2]];
    let mut ans = vec![];
    for i in 0..inc.len() {
        match inc[i] {
            "ProductOfNumbers" => {
                ans.push(None);
            }
            "add" => {
                s.add(args[i][0]);
                ans.push(None);
            }
            "getProduct" => {
                ans.push(Some(s.get_product(args[i][0])));
            }
            _ => (),
        }
    }
    assert_eq!(
        ans,
        vec![
            None,
            None,
            None,
            None,
            None,
            None,
            Some(20),
            Some(40),
            Some(0),
            None,
            Some(32)
        ]
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::VecDeque;
struct ProductOfNumbers {
    /// 记录三十二个大于1的数即可
    memo: VecDeque<(i32, i32)>,
    /// 指的是全部的数目而不是memo的长度
    len: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    fn new() -> Self {
        Self {
            memo: VecDeque::new(),
            len: 0,
        }
    }

    fn add(&mut self, num: i32) {
        match num {
            0 => {
                self.memo.clear();
                self.memo.push_back((0, self.len));
            }
            1 => (),
            _ => {
                self.memo.push_back((num, self.len));
                if self.memo.len() > 32 {
                    self.memo.pop_front();
                }
            }
        }
        self.len += 1;
    }

    fn get_product(&self, k: i32) -> i32 {
        // dbg!((k, &self.memo, self.len));
        let mut ans = 1;
        for (num, pos) in self.memo.iter().rev() {
            if pos + k < self.len {
                return ans;
            }
            ans *= num;
        }
        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// ["ProductOfNumbers","add","add","add","add","add","getProduct","getProduct","getProduct","add","getProduct"][[],[3],[0],[2],[5],[4],[2],[3],[4],[8],[2]]\n
// @lcpr case=end

 */
