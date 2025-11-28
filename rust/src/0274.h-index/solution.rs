// Created by bh4bxl at 2025/11/24 20:39
// leetgo: 1.4.15
// https://leetcode.com/problems/h-index/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations.clone();

        citations.sort();

        let mut res = i32::MIN;
        let len = citations.len() as i32;

        for (i, c) in citations.iter().enumerate() {
            let cur_h = (len - i as i32).min(*c);
            res = res.max(cur_h);
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let citations: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::h_index(citations).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
