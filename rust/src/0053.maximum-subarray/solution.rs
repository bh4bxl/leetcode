// Created by bh4bxl at 2025/07/29 15:37
// leetgo: 1.4.15
// https://leetcode.com/problems/maximum-subarray/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        let mut max = nums[0];

        dp[0] = nums[0];

        for i in 1..nums.len() {
            if dp[i - 1] < 0 {
                dp[i] = nums[i];
            } else {
                dp[i] = dp[i - 1] + nums[i];
            }

            max = std::cmp::max(max, dp[i]);
        }

        max
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let ans: i32 = Solution::max_sub_array(nums).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
