// Created by bh4bxl at 2025/12/02 19:14
// leetgo: 1.4.15
// https://leetcode.com/problems/minimum-height-trees/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;

        if n == 1 {
            return vec![0];
        }

        let mut graph = vec![vec![]; n];
        let mut degree = vec![0; n];

        for e in edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            graph[a].push(b);
            graph[b].push(a);
            degree[a] += 1;
            degree[b] += 1;
        }

        use std::collections::VecDeque;

        let mut queue = VecDeque::new();

        for i in 0..n {
            if degree[i] == 1 {
                queue.push_back(i);
            }
        }

        let mut remaining = n;

        while remaining > 2 {
            let size = queue.len();
            remaining -= size;

            for _ in 0..size {
                let leaf = queue.pop_front().unwrap();

                for &next in &graph[leaf] {
                    degree[next] -= 1;
                    if degree[next] == 1 {
                        queue.push_back(next);
                    }
                }
            }
        }

        queue.into_iter().map(|x| x as i32).collect()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let edges: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::find_min_height_trees(n, edges).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
