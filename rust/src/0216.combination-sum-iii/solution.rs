// Created by bh4bxl at 2025/10/27 20:18
// leetgo: 1.4.15
// https://leetcode.com/problems/combination-sum-iii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut temp = vec![];

        fn dfs(
            res: &mut Vec<Vec<i32>>,
            temp: &mut Vec<i32>,
            target: i32,
            idx: i32,
            sum: i32,
            k: usize,
        ) {
            if temp.len() == k && sum == target {
                res.push(temp.clone());
                return;
            }
            if sum < target && temp.len() < k {
                for i in idx..=9 {
                    temp.push(i);
                    dfs(res, temp, target, i + 1, sum + i, k);
                    temp.pop();
                }
            }
        }

        dfs(&mut res, &mut temp, n, 1, 0, k as usize);

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let k: i32 = deserialize(&read_line()?)?;
    let n: i32 = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::combination_sum3(k, n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
