// Created by bh4bxl at 2025/10/28 12:19
// leetgo: 1.4.15
// https://leetcode.com/problems/the-skyline-problem/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut events = vec![];

        for b in buildings {
            let (l, r, h) = (b[0], b[1], b[2]);
            events.push((l, -h));
            events.push((r, h));
        }

        events.sort_by(|a, b| {
            if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });

        let mut res = vec![];
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(0);
        let mut prev_height = 0;
        let mut removed = std::collections::HashMap::new();

        for (x, h) in events {
            if h < 0 {
                heap.push(-h);
            } else {
                *removed.entry(h).or_insert(0) += 1;
            }

            while let Some(&top) = heap.peek() {
                if *removed.get(&top).unwrap_or(&0) > 0 {
                    *removed.get_mut(&top).unwrap() -= 1;
                    if *removed.get(&top).unwrap() == 0 {
                        removed.remove(&top);
                    }
                    heap.pop();
                } else {
                    break;
                }
            }

            let curr_height = *heap.peek().unwrap();
            if curr_height != prev_height {
                res.push(vec![x, curr_height]);
                prev_height = curr_height;
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let buildings: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::get_skyline(buildings).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
