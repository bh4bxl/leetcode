// Created by bh4bxl at 2025/12/31 21:38
// leetgo: 1.4.16
// https://leetcode.com/problems/russian-doll-envelopes/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes.clone();

        // Sort: width ↑, height ↓
        envelopes.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        // LIS on heights
        let mut lis = vec![];

        for env in envelopes {
            let h = env[1];
            match lis.binary_search(&h) {
                Ok(pos) => lis[pos] = h,
                Err(pos) => {
                    if pos == lis.len() {
                        lis.push(h);
                    } else {
                        lis[pos] = h;
                    }
                }
            }
        }

        lis.len() as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let envelopes: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_envelopes(envelopes).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
