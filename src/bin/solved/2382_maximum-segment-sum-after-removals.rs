/*
 * @lc app=leetcode id=2382 lang=rust
 *
 * [2382] Maximum Segment Sum After Removals
 */

// @lc code=start
use std::collections::{BTreeSet, BinaryHeap, HashSet};

struct SegTree<T, F> {
    n: usize,
    data: Vec<T>,
    zero: T,
    merge: F,
}

impl<T, F> SegTree<T, F>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
{
    pub fn new(arr: &[T], zero: T, merge: F) -> Self {
        let mut n = 1;
        while n < arr.len() {
            n *= 2;
        }
        let mut st = Self {
            n,
            data: vec![zero; n * 2],
            zero,
            merge,
        };
        for (i, &val) in arr.iter().enumerate() {
            st.data[n + i] = val;
        }
        for i in (1..n).rev() {
            st.data[i] = (st.merge)(st.data[i * 2], st.data[i * 2 + 1]);
        }
        st
    }
    pub fn query(&self, mut l: usize, mut r: usize) -> T {
        l += self.n;
        r += self.n;
        let mut res_l = self.zero;
        let mut res_r = self.zero;
        while l <= r {
            if l % 2 == 1 {
                res_l = (self.merge)(res_l, self.data[l]);
                l += 1;
            }
            if r % 2 == 0 {
                res_r = (self.merge)(self.data[r], res_r);
                r -= 1;
            }
            l /= 2;
            r /= 2;
        }

        (self.merge)(res_l, res_r)
    }
}

impl Solution {
    pub fn maximum_segment_sum(nums: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64> {
        // 去除次数和数组长度一样，每次去除数不一样，因此最终一定是全0
        let nums = nums.into_iter().map(|i| i as i64).collect::<Vec<_>>();
        let st = SegTree::new(&nums, 0, |a, b| a + b);
        let mut segs_left = vec![None::<usize>; nums.len()];
        let mut segs_right = vec![None::<usize>; nums.len()];
        let mut max_segs = BinaryHeap::new();
        let mut res = vec![];
        for query in remove_queries.iter().rev() {
            res.push(*max_segs.peek().unwrap_or(&0));
            let query = *query as usize;
            let mut seg = (query, query);
            segs_left[query] = None;
            segs_right[query] = None;
            if query > 0 {
                if let Some(left) = segs_left[query - 1].take() {
                    seg.0 = left;
                }
            }
            if query < nums.len() - 1 {
                if let Some(right) = segs_right[query + 1].take() {
                    seg.1 = right;
                }
            }
            segs_left[seg.1] = Some(seg.0);
            segs_right[seg.0] = Some(seg.1);
            // dbg!(&segs_left, &segs_right);
            // dbg!(seg);
            max_segs.push(st.query(seg.0, seg.1));
        }

        res.into_iter().rev().collect()
    }
    #[allow(dead_code)]
    pub fn maximum_segment_sum_slow(nums: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64> {
        let nums = nums.into_iter().map(|i| i as i64).collect::<Vec<_>>();
        let st = SegTree::new(&nums, 0, |a, b| a + b);
        let mut segs = BTreeSet::new();
        segs.insert((0usize, nums.len() - 1));
        let mut max_segs = BinaryHeap::new();
        let seg = (0usize, nums.len() - 1);
        max_segs.push((st.query(seg.0, seg.1), seg));
        let mut removed_segs = HashSet::new();
        let mut res = vec![];
        for query in remove_queries {
            let query = query as usize;
            if let Some(seg) = segs.range(..(query + 1, usize::MAX)).next_back() {
                let (l, r) = *seg;
                if l <= query && query <= r {
                    segs.remove(&(l, r));
                    removed_segs.insert((l, r));
                    if l < query {
                        let seg = (l, query - 1);
                        segs.insert(seg);
                        max_segs.push((st.query(seg.0, seg.1), seg));
                    }
                    if query < r {
                        let seg = (query + 1, r);
                        segs.insert(seg);
                        max_segs.push((st.query(seg.0, seg.1), seg));
                    }
                }
            }
            let mut max_seg = 0;
            while let Some((max_sum, seg)) = max_segs.peek() {
                if removed_segs.contains(seg) {
                    max_segs.pop();
                } else {
                    max_seg = *max_sum as i64;
                    break;
                }
            }
            res.push(max_seg);
        }

        res
    }
}
// @lc code=end

struct Solution;
fn main() {
    // let st = SegTree::new(&[1, 2, 5, 6, 1], 0, |a, b| a + b);
    // for i in 0..=4 {
    //     println!("{}: {}", i, st.query(i, i));
    // }
    assert_eq!(
        Solution::maximum_segment_sum(vec![1, 2, 5, 6, 1], vec![0, 3, 2, 4, 1]),
        vec![14, 7, 2, 2, 0]
    );
    assert_eq!(
        Solution::maximum_segment_sum(vec![3, 2, 11, 1], vec![3, 2, 1, 0]),
        vec![16, 5, 3, 0]
    );
}
