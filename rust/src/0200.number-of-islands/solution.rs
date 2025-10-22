// Created by bh4bxl at 2025/10/20 13:40
// leetgo: 1.4.15
// https://leetcode.com/problems/number-of-islands/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

static DIR: [[i32; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let (height, width) = (grid.len(), grid[0].len());
        let mut used = vec![vec![false; width]; height];
        let mut count = 0;

        fn bfs(grid: &Vec<Vec<char>>, used: &mut Vec<Vec<bool>>, row: i32, col: i32) {
            if row < 0 || row as usize >= grid.len() || col < 0 || col as usize >= grid[0].len() {
                return;
            }
            if grid[row as usize][col as usize] == '0' || used[row as usize][col as usize] {
                return;
            }
            used[row as usize][col as usize] = true;
            for i in 0..4 {
                bfs(grid, used, row + DIR[i][0], col + DIR[i][1]);
            }
        }

        for i in 0..height {
            for j in 0..width {
                if grid[i][j] == '1' && !used[i][j] {
                    count += 1;
                    bfs(&grid, &mut used, i as i32, j as i32);
                }
            }
        }

        count
    }
}

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
fn main() -> Result<()> {
    let grid: Vec<Vec<char>> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::num_islands(grid).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
