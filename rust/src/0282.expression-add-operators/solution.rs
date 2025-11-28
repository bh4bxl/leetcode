// Created by bh4bxl at 2025/11/25 19:47
// leetgo: 1.4.15
// https://leetcode.com/problems/expression-add-operators/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        fn dfs(
            num: &str,
            target: i64,
            idx: usize,
            expr: &mut String,
            val: i64,
            last: i64,
            res: &mut Vec<String>,
        ) {
            if idx == num.len() {
                if val == target {
                    res.push(expr.clone());
                }
                return;
            }

            for i in idx..num.len() {
                if i != idx && &num[idx..idx + 1] == "0" {
                    break;
                }

                let part = &num[idx..=i];
                let n = part.parse::<i64>().unwrap();
                let len_before = expr.len();

                if idx == 0 {
                    expr.push_str(part);
                    dfs(num, target, i + 1, expr, n, n, res);
                    expr.truncate(len_before);
                } else {
                    expr.push('+');
                    expr.push_str(part);
                    dfs(num, target, i + 1, expr, val + n, n, res);
                    expr.truncate(len_before);

                    expr.push('-');
                    expr.push_str(part);
                    dfs(num, target, i + 1, expr, val - n, -n, res);
                    expr.truncate(len_before);

                    expr.push('*');
                    expr.push_str(part);
                    dfs(
                        num,
                        target,
                        i + 1,
                        expr,
                        val - last + last * n,
                        last * n,
                        res,
                    );
                    expr.truncate(len_before);
                }
            }
        }

        let mut res = vec![];
        let mut expr = String::new();
        dfs(&num, target as i64, 0, &mut expr, 0, 0, &mut res);

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let num: String = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: Vec<String> = Solution::add_operators(num, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
