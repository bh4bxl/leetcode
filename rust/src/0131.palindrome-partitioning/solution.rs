// Created by bh4bxl at 2025/09/15 12:37
// leetgo: 1.4.15
// https://leetcode.com/problems/palindrome-partitioning/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = vec![];
        let mut path = vec![];

        fn is_palindrome(s: &str) -> bool {
            let bytes = s.as_bytes();
            let len = bytes.len();
            if len == 0 || len == 1 {
                return true;
            }

            let (mut l, mut r) = (0, len - 1);
            while l < r {
                if bytes[l] != bytes[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }

            true
        }

        fn backtrace<'a>(
            s: &'a str,
            start: usize,
            path: &mut Vec<String>,
            res: &mut Vec<Vec<String>>,
        ) {
            if start == s.len() {
                res.push(path.clone());
                return;
            }

            for end in start + 1..=s.len() {
                let substr = &s[start..end];
                if is_palindrome(substr) {
                    path.push(substr.to_string());
                    backtrace(s, end, path, res);
                    path.pop();
                }
            }
        }

        backtrace(&s, 0, &mut path, &mut res);

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: Vec<Vec<String>> = Solution::partition(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
