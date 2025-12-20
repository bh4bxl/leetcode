// Created by bh4bxl at 2025/12/17 14:23
// leetgo: 1.4.15
// https://leetcode.com/problems/increasing-triplet-subsequence/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut m1, mut m2) = (i32::MAX, i32::MAX);

        for num in nums {
            if m1 >= num {
                m1 = num;
            } else if m2 >= num {
                m2 = num;
            } else {
                return true;
            }
        }

        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: bool = Solution::increasing_triplet(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
