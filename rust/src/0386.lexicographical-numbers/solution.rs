// Created by bh4bxl at 2026/01/20 17:48
// leetgo: 1.4.16
// https://leetcode.com/problems/lexicographical-numbers/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        fn dfs(curr: i32, n: i32, res: &mut Vec<i32>) {
            if curr > n {
                return;
            }

            res.push(curr);
            for i in 0..=9 {
                let next = curr * 10 + i;
                if next > n {
                    break;
                }
                dfs(next, n, res);
            }
        }

        let mut res = Vec::with_capacity(n as usize);

        for i in 1..=9 {
            if i > n {
                break;
            }

            dfs(i, n, &mut res);
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::lexical_order(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
