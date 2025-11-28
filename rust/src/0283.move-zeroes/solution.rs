// Created by bh4bxl at 2025/11/25 20:01
// leetgo: 1.4.15
// https://leetcode.com/problems/move-zeroes/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut insert_pos = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[insert_pos] = nums[i];
                insert_pos += 1;
            }
        }

        for i in insert_pos..nums.len() {
            nums[i] = 0;
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let mut nums: Vec<i32> = deserialize(&read_line()?)?;
    Solution::move_zeroes(&mut nums);
    let ans: Vec<i32> = nums.into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
