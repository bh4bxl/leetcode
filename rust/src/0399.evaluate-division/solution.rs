// Created by bh4bxl at 2026/01/28 18:09
// leetgo: 1.4.16
// https://leetcode.com/problems/evaluate-division/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();

        // build graph
        for (eq, &val) in equations.iter().zip(values.iter()) {
            let (a, b) = (&eq[0], &eq[1]);
            graph.entry(a.clone()).or_default().push((b.clone(), val));
            graph
                .entry(b.clone())
                .or_default()
                .push((a.clone(), 1.0 / val));
        }

        // dfs helper
        fn dfs(
            cur: &str,
            target: &str,
            product: f64,
            graph: &HashMap<String, Vec<(String, f64)>>,
            visited: &mut HashSet<String>,
        ) -> Option<f64> {
            if cur == target {
                return Some(product);
            }

            visited.insert(cur.to_string());

            if let Some(neighbors) = graph.get(cur) {
                for (next, weight) in neighbors {
                    if !visited.contains(next) {
                        if let Some(res) = dfs(next, target, product * weight, graph, visited) {
                            return Some(res);
                        }
                    }
                }
            }

            None
        }

        // answer queries
        let mut res = vec![];

        for q in queries {
            let (a, b) = (&q[0], &q[1]);

            if !graph.contains_key(a) || !graph.contains_key(b) {
                res.push(-1.0);
            } else {
                let mut visited = HashSet::new();
                match dfs(a, b, 1.0, &graph, &mut visited) {
                    Some(v) => res.push(v),
                    None => res.push(-1.0),
                }
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let equations: Vec<Vec<String>> = deserialize(&read_line()?)?;
    let values: Vec<f64> = deserialize(&read_line()?)?;
    let queries: Vec<Vec<String>> = deserialize(&read_line()?)?;
    let ans: Vec<f64> = Solution::calc_equation(equations, values, queries).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
