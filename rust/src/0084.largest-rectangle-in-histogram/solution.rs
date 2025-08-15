// Created by bh4bxl at 2025/08/14 13:33
// leetgo: 1.4.15
// https://leetcode.com/problems/largest-rectangle-in-histogram/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut max_area = 0;
        let mut extended = heights.clone();
        extended.push(0);

        for i in 0..extended.len() {
            while let Some(&top) = stack.last() {
                if extended[i] < extended[top] {
                    let height = extended[top];
                    stack.pop();
                    let width = if let Some(&prev_top) = stack.last() {
                        i - prev_top - 1
                    } else {
                        i
                    };
                    max_area = std::cmp::max(max_area, height * width as i32);
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        max_area
    }
}

// @lc code=end

fn main() -> Result<()> {
    let heights: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::largest_rectangle_area(heights).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
