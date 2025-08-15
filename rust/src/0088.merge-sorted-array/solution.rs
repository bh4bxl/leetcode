// Created by bh4bxl at 2025/08/14 20:23
// leetgo: 1.4.15
// https://leetcode.com/problems/merge-sorted-array/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in 0..n as usize {
            nums1[m as usize + i] = nums2[i];
        }
        nums1.sort();
    }
}

// @lc code=end

fn main() -> Result<()> {
    let mut nums1: Vec<i32> = deserialize(&read_line()?)?;
    let m: i32 = deserialize(&read_line()?)?;
    let mut nums2: Vec<i32> = deserialize(&read_line()?)?;
    let n: i32 = deserialize(&read_line()?)?;
    Solution::merge(&mut nums1, m, &mut nums2, n);
    let ans: Vec<i32> = nums1.into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
