// Created by bh4bxl at 2025/12/17 14:08
// leetgo: 1.4.15
// https://leetcode.com/problems/reconstruct-itinerary/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashMap};

        fn dfs(
            airport: String,
            graph: &mut HashMap<String, BinaryHeap<Reverse<String>>>,
            route: &mut Vec<String>,
        ) {
            while let Some(heap) = graph.get_mut(&airport) {
                if heap.is_empty() {
                    break;
                }

                let next = heap.pop().unwrap().0;
                dfs(next, graph, route);
            }

            route.push(airport);
        }

        // Build graph with mini-heaps
        let mut graph = HashMap::new();
        for t in tickets {
            graph
                .entry(t[0].clone())
                .or_insert_with(BinaryHeap::new)
                .push(Reverse(t[1].clone()));
        }

        let mut route = vec![];

        dfs("JFK".to_string(), &mut graph, &mut route);

        route.reverse();

        route
    }
}

// @lc code=end

fn main() -> Result<()> {
    let tickets: Vec<Vec<String>> = deserialize(&read_line()?)?;
    let ans: Vec<String> = Solution::find_itinerary(tickets).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
