// Created by bh4bxl at 2025/07/21 14:56
// leetgo: 1.4.15
// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut prev: i32 = 0;
        let mut prev_pos: usize = 1;

        if nums.is_empty() {
            return 0;
        }

        prev = nums[0];

        for i in 0..nums.len() {
            if nums[i] == prev {
                continue;
            }
            prev = nums[i];
            nums[prev_pos] = prev;
            prev_pos += 1;

        }

        return prev_pos as i32;
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let ans: i32 = Solution::remove_duplicates(nums).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
