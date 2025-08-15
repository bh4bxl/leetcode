// Created by bh4bxl at 2025/08/14 19:56
// leetgo: 1.4.15
// https://leetcode.com/problems/scramble-string/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        Self::dfs(
            s1.as_str(),
            s2.as_str(),
            &mut std::collections::HashMap::new(),
        )
    }

    fn dfs(a: &str, b: &str, memo: &mut std::collections::HashMap<(String, String), bool>) -> bool {
        if a == b {
            return true;
        }
        let key = (a.to_string(), b.to_string());
        if let Some(&catched) = memo.get(&key) {
            return catched;
        }
        if a.len() != b.len() || {
            let mut ca = [0; 26];
            for (ca_s, cb_s) in a.bytes().zip(b.bytes()) {
                ca[(ca_s - b'a') as usize] += 1;
                ca[(cb_s - b'a') as usize] -= 1;
            }
            ca.iter().any(|&c| c != 0)
        } {
            memo.insert(key.clone(), false);
            return false;
        }

        let n = a.len();
        for i in 1..n {
            if Self::dfs(&a[..i], &b[..i], memo) && Self::dfs(&a[i..], &b[i..], memo) {
                memo.insert(key.clone(), true);
                return true;
            }
            if Self::dfs(&a[..i], &b[n - i..], memo) && Self::dfs(&a[i..], &b[..n - i], memo) {
                memo.insert(key.clone(), true);
                return true;
            }
        }

        memo.insert(key.clone(), false);
        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s1: String = deserialize(&read_line()?)?;
    let s2: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_scramble(s1, s2).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
