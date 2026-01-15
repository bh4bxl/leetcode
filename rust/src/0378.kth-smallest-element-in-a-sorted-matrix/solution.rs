// Created by bh4bxl at 2026/01/14 17:33
// leetgo: 1.4.16
// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let mut left = matrix[0][0];
        let mut right = matrix[n - 1][n - 1];

        while left < right {
            let mid = left + (right - left) / 2;
            let mut count = 0;
            let (mut row, mut col) = (n as i32 - 1, 0);

            while row >= 0 && col < n {
                if matrix[row as usize][col] <= mid {
                    count += row + 1;
                    col += 1;
                } else {
                    row -= 1;
                }
            }

            if count < k {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}

// @lc code=end

fn main() -> Result<()> {
    let matrix: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::kth_smallest(matrix, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
