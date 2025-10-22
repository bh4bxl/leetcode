// Created by bh4bxl at 2025/10/21 15:20
// leetgo: 1.4.15
// https://leetcode.com/problems/course-schedule-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut graph = vec![vec![]; n];
        let mut indegree = vec![0; n];

        for edge in prerequisites {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            graph[b].push(a);
            indegree[a] += 1;
        }

        let mut queue = std::collections::VecDeque::new();
        for i in 0..n {
            if indegree[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut order = vec![];

        while let Some(course) = queue.pop_front() {
            order.push(course as i32);
            for &next in &graph[course] {
                indegree[next] -= 1;
                if indegree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        if order.len() == n { order } else { vec![] }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let num_courses: i32 = deserialize(&read_line()?)?;
    let prerequisites: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::find_order(num_courses, prerequisites).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
