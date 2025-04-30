/*
 * @lc app=leetcode id=1847 lang=rust
 *
 * [1847] Closest Room
 */

use leetcode_rs::vec2d;

// @lc code=start
use std::collections::BTreeSet;
impl Solution {
    pub fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // 离线查询，因而对查询排序即可
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        struct Query {
            size: i32,
            id: i32,
            index: usize,
        }
        let mut queries = queries
            .iter()
            .enumerate()
            .map(|(index, q)| Query {
                size: q[1],
                id: q[0],
                index,
            })
            .collect::<Vec<_>>();
        queries.sort_by_key(|q| q.size);
        let mut rooms = rooms;
        rooms.sort_by_key(|room| room[1]);
        let mut cur_rooms = BTreeSet::new();
        let mut ans = vec![0; queries.len()];
        let mut rooms = rooms.into_iter().rev().peekable();
        for Query { id, size, index } in queries.into_iter().rev() {
            // 增加房间
            while let Some(room) = rooms.peek().filter(|r| r[1] >= size) {
                cur_rooms.insert(room[0]);
                let _ = rooms.next();
            }
            // dbg!(&rooms, &cur_rooms);
            let lower = cur_rooms.range(..=id).next_back();
            let upper = cur_rooms.range(id..).next();
            // dbg!(lower, upper, id);
            let closest = match (lower, upper) {
                (None, None) => -1,
                (Some(r), None) | (None, Some(r)) => *r,
                (Some(&l), Some(&r)) => {
                    if id - l <= r - id {
                        l
                    } else {
                        r
                    }
                }
            };
            // dbg!(closest);
            ans[index] = closest;
        }

        ans
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(
        Solution::closest_room(
            vec2d![[2, 2], [1, 2], [3, 2]],
            vec2d![[3, 1], [3, 3], [5, 2]]
        ),
        vec![3, -1, 3]
    );
    assert_eq!(
        Solution::closest_room(
            vec2d![[1, 4], [2, 3], [3, 5], [4, 1], [5, 2]],
            vec2d![[2, 3], [2, 4], [2, 5]]
        ),
        vec![2, 1, 3]
    );
}
