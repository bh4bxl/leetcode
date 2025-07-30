// Created by bh4bxl at 2025/07/29 22:22
// leetgo: 1.4.15
// https://leetcode.com/problems/jump-game/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut reach = nums[0];

        for i in 0..nums.len() {
            if reach < i as i32 {
                return false;
            }
            reach =std::cmp::max(reach, i as i32 + nums[i]);
        }

        true
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let ans: bool = Solution::can_jump(nums).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
