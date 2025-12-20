// Created by bh4bxl at 2025/12/18 14:45
// leetgo: 1.4.15
// https://leetcode.com/problems/self-crossing/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        let n = distance.len();

        if n < 4 {
            return false;
        }

        for i in 3..n {
            // Case 1: current line crosses the line 3 steps back
            if distance[i] >= distance[i - 2] && distance[i - 1] <= distance[i - 3] {
                return true;
            }

            // Case 2: current line meets the line 4 steps back
            if i >= 4
                && distance[i - 1] == distance[i - 3]
                && distance[i] + distance[i - 4] >= distance[i - 2]
            {
                return true;
            }

            // Case 3: current line crosses the line 5 steps back
            if i >= 5
                && distance[i - 2] >= distance[i - 4]
                && distance[i - 1] <= distance[i - 3]
                && distance[i] >= distance[i - 2] - distance[i - 4]
                && distance[i - 1] + distance[i - 5] >= distance[i - 3]
            {
                return true;
            }
        }

        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let distance: Vec<i32> = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_self_crossing(distance).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
