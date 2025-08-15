// Created by bh4bxl at 2025/08/14 14:53
// leetgo: 1.4.15
// https://leetcode.com/problems/maximal-rectangle/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let (rows, cols) = (matrix.len(), matrix[0].len());
        let mut heights = vec![0; cols];
        let mut max_area = 0;

        for i in 0..rows {
            for j in 0..cols {
                if matrix[i][j] == '1' {
                    heights[j] += 1;
                } else {
                    heights[j] = 0;
                }
            }
            max_area = std::cmp::max(max_area, Self::largest_rectangle_area(&heights));
        }

        max_area
    }

    fn largest_rectangle_area(heights: &Vec<i32>) -> i32 {
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
    let matrix: Vec<Vec<char>> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::maximal_rectangle(matrix).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
