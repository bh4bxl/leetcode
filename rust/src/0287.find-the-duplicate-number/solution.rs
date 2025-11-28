// Created by bh4bxl at 2025/11/25 20:19
// leetgo: 1.4.15
// https://leetcode.com/problems/find-the-duplicate-number/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut cnts = vec![0; nums.len()];

        for n in nums {
            cnts[n as usize] += 1;
            if cnts[n as usize] > 1 {
                return n;
            }
        }

        0
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::find_duplicate(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
