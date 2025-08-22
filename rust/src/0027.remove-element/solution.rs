// Created by bh4bxl at 2025/07/21 16:04
// leetgo: 1.4.15
// https://leetcode.com/problems/remove-element/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut last_pos: usize = 0;

        for i in 0..nums.len() {
            if nums[i] == val {
                continue;
            }

            nums[last_pos] = nums[i];
            last_pos += 1;
        }

        return last_pos as i32;
    }
}

// @lc code=end

fn main() -> Result<()> {
    let mut nums: Vec<i32> = deserialize(&read_line()?)?;
    let val: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::remove_element(&mut nums, val).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
