// Created by bh4bxl at 2025/11/03 20:10
// leetgo: 1.4.15
// https://leetcode.com/problems/summary-ranges/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return vec![];
        }

        if nums.len() == 1 {
            return vec![nums[0].to_string()];
        }

        let mut start = 0;
        let mut res = vec![];

        for (i, num) in nums.iter().enumerate() {
            if i == 0 {
                start = *num;
                continue;
            }
            if num - nums[i - 1] != 1 {
                if nums[i - 1] == start {
                    res.push(start.to_string());
                } else {
                    let mut s = start.to_string();
                    s.push_str("->");
                    s.push_str(&nums[i - 1].to_string());
                    res.push(s);
                }
                start = *num;
            }
            if i == nums.len() - 1 {
                if *num == start {
                    res.push(start.to_string());
                } else {
                    let mut s = start.to_string();
                    s.push_str("->");
                    s.push_str(&num.to_string());
                    res.push(s);
                }
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<String> = Solution::summary_ranges(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
