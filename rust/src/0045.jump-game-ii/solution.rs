// Created by bh4bxl at 2025/07/28 16:28
// leetgo: 1.4.15
// https://leetcode.com/problems/jump-game-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut end = 0;
        let mut max_pos = 0;
        let mut steps = 0;

        for i in 0..nums.len() - 1 {
            max_pos = std::cmp::max(max_pos, nums[i] as usize + i);

            if i == end {
                end = max_pos;
                steps += 1;
            }
        }

        steps
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let ans: i32 = Solution::jump(nums).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
