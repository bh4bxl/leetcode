// Created by bh4bxl at 2026/01/13 21:41
// leetgo: 1.4.16
// https://leetcode.com/problems/wiggle-subsequence/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let (mut up, mut down) = (1, 1);

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                up = down + 1;
            } else if nums[i] < nums[i - 1] {
                down = up + 1;
            }
        }

        up.max(down)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::wiggle_max_length(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
