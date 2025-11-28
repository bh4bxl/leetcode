// Created by bh4bxl at 2025/11/24 21:01
// leetgo: 1.4.15
// https://leetcode.com/problems/h-index-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let len = citations.len() as i32;
        let (mut left, mut right) = (0, len - 1);
        let mut res = 0;

        while left <= right {
            let mid = left + (right - left) / 2;
            let papers = len - mid;

            if citations[mid as usize] >= papers {
                res = papers;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
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
