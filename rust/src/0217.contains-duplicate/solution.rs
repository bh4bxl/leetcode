// Created by bh4bxl at 2025/10/28 12:09
// leetgo: 1.4.15
// https://leetcode.com/problems/contains-duplicate/

use core::num;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();

        for n in nums {
            if !set.insert(n) {
                return true;
            }
        }

        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: bool = Solution::contains_duplicate(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
