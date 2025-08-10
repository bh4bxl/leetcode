// Created by bh4bxl at 2025/08/06 15:57
// leetgo: 1.4.15
// https://leetcode.com/problems/simplify-path/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let folders: Vec<&str> = path.split('/').collect();
        let mut path = vec![];
        let mut res = String::new();

        for folder in folders {
            if folder == ".." {
                if !path.is_empty() {
                    path.pop();
                }
                continue;
            }
            if folder == "." || folder.len() == 0 {
                continue;
            }
            path.push(folder);
        }

        for s in path {
            res.push('/');
            res.push_str(&s.to_string());
        }

        if res.len() == 0 {
            String::from("/")
        } else {
            res
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let path: String = deserialize(&read_line()?)?;
    let ans: String = Solution::simplify_path(path).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
