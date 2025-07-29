// Created by bh4bxl at 2025/07/27 21:40
// leetgo: 1.4.15
// https://leetcode.com/problems/trapping-rain-water/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 2 {
            return 0;
        }

        let mut sum = 0;
        let len = height.len();
        let (mut max_left, mut max_right) = (vec![0; len], vec![0; len]);

        for i in 1..len - 1 {
            max_left[i] = std::cmp::max(max_left[i - 1], height[i - 1]);
        }

        let mut j = len as i32 - 2;
        while j >= 0 {
            max_right[j as usize] =
                std::cmp::max(max_right[j as usize + 1], height[j as usize + 1]);
            j -= 1;
        }

        for i in 1..len - 1 {
            let min = std::cmp::min(max_left[i], max_right[i]);
            if min > height[i] {
                sum += min - height[i];
            }
        }

        sum
    }
}

// @lc code=end

fn main() -> Result<()> {
	let height: Vec<i32> = deserialize(&read_line()?)?;
	let ans: i32 = Solution::trap(height).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
