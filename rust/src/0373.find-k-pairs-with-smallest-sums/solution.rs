// Created by bh4bxl at 2026/01/12 19:57
// leetgo: 1.4.16
// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();

        // (sum, i, j)
        heap.push(Reverse((nums1[0] + nums2[0], 0usize, 0usize)));
        visited.insert((0usize, 0usize));

        while let Some(Reverse((_sum, i, j))) = heap.pop() {
            res.push(vec![nums1[i], nums2[j]]);
            if res.len() == k as usize {
                break;
            }

            // Move down
            if i + 1 < nums1.len() && visited.insert((i + 1, j)) {
                heap.push(Reverse((nums1[i + 1] + nums2[j], i + 1, j)));
            }

            // Move right
            if j + 1 < nums2.len() && visited.insert((i, j + 1)) {
                heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums1: Vec<i32> = deserialize(&read_line()?)?;
    let nums2: Vec<i32> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::k_smallest_pairs(nums1, nums2, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
