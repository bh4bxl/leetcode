// Created by bh4bxl at 2025/10/20 15:19
// leetgo: 1.4.15
// https://leetcode.com/problems/isomorphic-strings/

use std::usize;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (mut s_arr, mut t_arr) = ([usize::MAX; 256], [usize::MAX; 256]);
        let len = s.len();
        let s_l: Vec<u8> = s.into_bytes();
        let t_l: Vec<u8> = t.into_bytes();

        for i in 0..len {
            let s_c = s_l[i] as usize;
            let t_c = t_l[i] as usize;
            if s_arr[s_c] != t_arr[t_c] {
                return false;
            }
            s_arr[s_c] = i;
            t_arr[t_c] = i;
        }

        true
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let t: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_isomorphic(s, t).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
