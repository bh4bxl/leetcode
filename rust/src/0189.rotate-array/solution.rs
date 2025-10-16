// Created by bh4bxl at 2025/10/15 19:56
// leetgo: 1.4.15
// https://leetcode.com/problems/rotate-array/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let r = (k as usize) % len;
        if r == 0 {
            return;
        }
        let mut l1 = nums.split_off(len - r);
        l1.extend(nums.iter());
        *nums = l1.clone();
    }
}

// @lc code=end

fn main() -> Result<()> {
    let mut nums: Vec<i32> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    Solution::rotate(&mut nums, k);
    let ans: Vec<i32> = nums.into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
