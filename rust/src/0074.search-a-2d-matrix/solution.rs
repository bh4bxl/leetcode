// Created by bh4bxl at 2025/08/08 10:58
// leetgo: 1.4.15
// https://leetcode.com/problems/search-a-2d-matrix/

use std::usize;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (cow, col) = (matrix.len(), matrix[0].len());
        let (mut min, mut max) = (0, cow - 1);
        let mut line = usize::MAX;
        while min <= max {
            let mid = (min + max) / 2;
            if target > matrix[mid][0] && target < matrix[mid][col - 1] {
                line = mid;
                break;
            } else if target == matrix[mid][0] || target == matrix[mid][col - 1] {
                return true;
            } else if target < matrix[mid][0] {
                if mid == 0 {
                    return false;
                }
                max = mid - 1;
            } else {
                if mid == cow - 1 {
                    return false;
                }
                min = mid + 1;
            }
        }

        if line == usize::MAX {
            return false;
        }

        (min, max) = (0, col - 1);
        while min <= max {
            let mid = (min + max) / 2;
            if target == matrix[line][mid] {
                return true;
            } else if target < matrix[line][mid] {
                max = mid - 1;
            } else {
                min = mid + 1;
            }
        }
        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let matrix: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::search_matrix(matrix, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
