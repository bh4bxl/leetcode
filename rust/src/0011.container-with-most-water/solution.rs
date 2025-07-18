// Created by bh4bxl at 2025/07/14 14:34
// leetgo: 1.4.14
// https://leetcode.com/problems/container-with-most-water/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut l = 0;
        let mut r = height.len() - 1;

        while l < r {
            max_area = std::cmp::max(
                max_area,
                std::cmp::min(height[l], height[r]) * (r - l) as i32
            );
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1
            }
        }

        return max_area;
    }
}

// @lc code=end

fn main() -> Result<()> {
	let height: Vec<i32> = deserialize(&read_line()?)?;
	let ans: i32 = Solution::max_area(height).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
