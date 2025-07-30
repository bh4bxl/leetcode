// Created by bh4bxl at 2025/07/29 15:04
// leetgo: 1.4.15
// https://leetcode.com/problems/n-queens-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut cols = vec![false; n as usize];
        let mut d1 = vec![false; n as usize * 2];
        let mut d2 = vec![false; n as usize * 2];
        let mut count = 0;

        return Self::backtrace(0, &mut cols, &mut d1, &mut d2, n, &mut count);
    }

    fn backtrace(
        row: i32,
        cols: &mut Vec<bool>,
        d1: &mut Vec<bool>,
        d2: &mut Vec<bool>,
        n: i32,
        count: &mut i32
    ) -> i32 {
        if row == n {
            *count += 1;
        }

        for col in 0..n {
            let id1 = (row - col + n) as usize;
            let id2 = (row + col) as usize;
            if cols[col as usize] || d1[id1] || d2[id2] {
                continue;
            }
            cols[col as usize] = true;
            d1[id1] = true;
            d2[id2] = true;
            *count = Self::backtrace(row + 1, cols, d1, d2, n, count);
            cols[col as usize] = false;
            d1[id1] = false;
            d2[id2] = false;
        }

       *count
    }
}

// @lc code=end

fn main() -> Result<()> {
	let n: i32 = deserialize(&read_line()?)?;
	let ans: i32 = Solution::total_nqueens(n).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
