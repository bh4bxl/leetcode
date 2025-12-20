// Created by bh4bxl at 2025/12/16 23:25
// leetgo: 1.4.15
// https://leetcode.com/problems/patching-array/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut miss = 1i64;
        let mut i = 0;
        let mut patches = 0;
        let n = n as i64;

        while miss <= n {
            if i < nums.len() && nums[i] as i64 <= miss {
                miss += nums[i] as i64;
                i += 1;
            } else {
                // Patch miss itself
                miss += miss;
                patches += 1;
            }
        }

        patches
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::min_patches(nums, n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
