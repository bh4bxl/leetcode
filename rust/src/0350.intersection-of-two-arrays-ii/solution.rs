// Created by bh4bxl at 2025/12/29 19:10
// leetgo: 1.4.16
// https://leetcode.com/problems/intersection-of-two-arrays-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        let (mut idx1, mut idx2) = (0, 0);
        let mut res = vec![];

        nums1.sort();
        nums2.sort();

        while idx1 < nums1.len() && idx2 < nums2.len() {
            let (a, b) = (nums1[idx1], nums2[idx2]);
            idx1 += 1;
            idx2 += 1;
            if a == b {
                res.push(a);
            } else if a < b {
                idx2 -= 1;
            } else {
                idx1 -= 1;
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums1: Vec<i32> = deserialize(&read_line()?)?;
    let nums2: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::intersect(nums1, nums2).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
