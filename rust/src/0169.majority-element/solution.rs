// Created by bh4bxl at 2025/10/08 16:41
// leetgo: 1.4.15
// https://leetcode.com/problems/majority-element/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut nums_m = nums.clone();

        nums_m.sort();

        nums_m[nums.len() / 2]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::majority_element(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
