// Created by bh4bxl at 2025/11/03 10:34
// leetgo: 1.4.15
// https://leetcode.com/problems/rectangle-area/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let total = (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1);

        if bx1 > ax2 || bx2 < ax1 || by1 > ay2 || by2 < ay1 {
            return total;
        }

        let height = by2.min(ay2) - ay1.max(by1);
        let width = bx2.min(ax2) - ax1.max(bx1);

        total - height * width
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ax1: i32 = deserialize(&read_line()?)?;
    let ay1: i32 = deserialize(&read_line()?)?;
    let ax2: i32 = deserialize(&read_line()?)?;
    let ay2: i32 = deserialize(&read_line()?)?;
    let bx1: i32 = deserialize(&read_line()?)?;
    let by1: i32 = deserialize(&read_line()?)?;
    let bx2: i32 = deserialize(&read_line()?)?;
    let by2: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
