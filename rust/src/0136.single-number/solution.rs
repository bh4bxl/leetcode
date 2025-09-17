// Created by bh4bxl at 2025/09/16 13:54
// leetgo: 1.4.15
// https://leetcode.com/problems/single-number/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut nums_m = nums.clone();

        nums_m.sort();

        let mut i = 0;

        while i < nums.len() - 1 {
            if nums_m[i] != nums_m[i + 1] {
                break;
            }
            while nums_m[i] == nums_m[i + 1] {
                i += 1;
            }
            i += 1;
        }

        nums_m[i]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::single_number(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
