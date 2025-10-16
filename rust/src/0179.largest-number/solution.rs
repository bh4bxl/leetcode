// Created by bh4bxl at 2025/10/14 15:04
// leetgo: 1.4.15
// https://leetcode.com/problems/largest-number/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums_m = nums.clone();
        nums_m.sort_by(|a, b| {
            let s1 = a.to_string() + &b.to_string();
            let s2 = b.to_string() + &a.to_string();
            s2.parse::<i128>()
                .unwrap()
                .cmp(&s1.parse::<i128>().unwrap())
        });

        if nums_m[0] == 0 {
            return "0".to_string();
        }

        let mut res = String::new();

        for n in nums_m {
            res.push_str(&n.to_string());
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: String = Solution::largest_number(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
