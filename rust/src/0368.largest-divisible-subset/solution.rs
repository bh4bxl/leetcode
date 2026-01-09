// Created by bh4bxl at 2026/01/06 13:28
// leetgo: 1.4.16
// https://leetcode.com/problems/largest-divisible-subset/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        nums.sort_unstable();
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut prev = vec![-1; n];
        let mut max_len = 1;
        let mut max_idx = 0;

        for i in 0..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i] {
                    dp[i] = dp[j] + 1;
                    prev[i] = j as i32;
                }
            }

            if dp[i] > max_len {
                max_len = dp[i];
                max_idx = i;
            }
        }

        // Reconstruct result
        let mut res = vec![];
        let mut i = max_idx as i32;
        while i != -1 {
            res.push(nums[i as usize]);
            i = prev[i as usize];
        }

        res.reverse();

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::largest_divisible_subset(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
