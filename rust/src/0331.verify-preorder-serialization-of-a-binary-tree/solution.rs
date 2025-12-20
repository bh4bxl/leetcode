// Created by bh4bxl at 2025/12/17 14:00
// leetgo: 1.4.15
// https://leetcode.com/problems/verify-preorder-serialization-of-a-binary-tree/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut slots = 1;

        for node in preorder.split(',') {
            // one slot is used
            slots -= 1;
            if slots < 0 {
                return false;
            }

            // non-null node creats 2 slots
            if node != "#" {
                slots += 2;
            }
        }

        slots == 0
    }
}

// @lc code=end

fn main() -> Result<()> {
    let preorder: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_valid_serialization(preorder).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
