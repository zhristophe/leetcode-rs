/*
 * @lc app=leetcode.cn id=2709 lang=rust
 * @lcpr version=30204
 *
 * [2709] 最大公约数遍历
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::can_traverse_all_pairs(vec![2, 3, 6]), true);
    assert_eq!(Solution::can_traverse_all_pairs(vec![1]), true);
}
// @lcpr-template-end
// @lc code=start
use std::collections::{HashMap, HashSet};
struct UnionFind {
    map: HashMap<i32, i32>,
}
impl UnionFind {
    fn find(&mut self, x: i32) -> i32 {
        let parent = self.map[&x];
        if parent != x {
            let root = self.find(parent);
            self.map.insert(x, root);
        }
        self.map[&x]
    }
    fn union(&mut self, x: i32, y: i32) {
        let x = self.find(x);
        let y = self.find(y);
        if x != y {
            self.map.insert(x, y);
        }
    }
}
impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return true;
        }
        let maxn = *nums.iter().max().unwrap();
        let mut primes = vec![];
        let mut map = HashMap::new();
        for i in 2..=maxn {
            let mut f = true;
            for &p in &primes {
                if i % p == 0 {
                    map.insert(i, p);
                    f = false;
                    break;
                }
            }
            if f {
                primes.push(i);
                map.insert(i, i);
            }
        }
        // dbg!(&map);
        let mut union = UnionFind {
            map: HashMap::from_iter(primes.iter().map(|&p| (p, p))),
        };
        let mut ps = HashSet::new();
        for n in nums {
            if n <= 1 {
                return false;
            }
            let p = map[&n];
            ps.insert(p);
            let mut n = n / p;
            while n != 1 {
                let q = map[&n];
                ps.insert(q);
                // dbg!(p, q);
                union.union(p, q);
                n /= q;
            }
        }
        let root = union.find(*ps.iter().next().unwrap());
        // dbg!(&ps);
        // dbg!(&union.map);
        ps.iter().all(|p| union.find(*p) == root)
    }
}
// @lc code=end

/*
// @lcpr case=start
// [2,3,6]\n
// @lcpr case=end

// @lcpr case=start
// [3,9,5]\n
// @lcpr case=end

// @lcpr case=start
// [4,3,12,8]\n
// @lcpr case=end

 */
