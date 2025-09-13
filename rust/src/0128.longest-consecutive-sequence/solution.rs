// Created by bh4bxl at 2025/09/12 12:26
// leetgo: 1.4.15
// https://leetcode.com/problems/longest-consecutive-sequence/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut nums_m = nums.clone();
        let mut con_seq = 1;
        let mut curr_seq = 1;

        nums_m.sort();

        for i in 1..nums.len() {
            let diff = nums_m[i] - nums_m[i - 1];
            match diff {
                0 => (),
                1 => curr_seq += 1,
                _ => {
                    con_seq = con_seq.max(curr_seq);
                    curr_seq = 1
                }
            }
        }

        con_seq.max(curr_seq)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::longest_consecutive(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
