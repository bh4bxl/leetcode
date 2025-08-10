// Created by bh4bxl at 2025/08/10 11:57
// leetgo: 1.4.15
// https://leetcode.com/problems/combinations/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut temp = vec![];

        Self::get_combine(1, n, k, &mut temp, &mut res);

        res
    }

    fn get_combine(start: i32, n: i32, k: i32, temp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if temp.len() == k as usize {
            res.push(temp.clone());
            return;
        }

        for i in start..=n {
            temp.push(i);
            Self::get_combine(i + 1, n, k, temp, res);
            temp.pop();
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::combine(n, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
