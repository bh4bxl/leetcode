// Created by bh4bxl at 2025/09/16 14:15
// leetgo: 1.4.15
// https://leetcode.com/problems/single-number-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut nums_m = nums.clone();
        nums_m.sort();
        if nums.len() < 3 {
            return nums_m[0];
        }

        let mut i = 0;
        while i < nums.len() - 1 {
            if nums_m[i] != nums_m[i + 1] {
                break;
            }
            i += 1;
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
