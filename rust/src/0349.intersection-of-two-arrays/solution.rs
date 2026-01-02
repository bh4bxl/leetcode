// Created by bh4bxl at 2025/12/29 19:03
// leetgo: 1.4.16
// https://leetcode.com/problems/intersection-of-two-arrays/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];

        for n in nums2 {
            if nums1.contains(&n) && !res.contains(&n) {
                res.push(n);
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums1: Vec<i32> = deserialize(&read_line()?)?;
    let nums2: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::intersection(nums1, nums2).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
