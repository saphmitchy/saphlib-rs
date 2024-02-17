// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shortest_path

use graph_base::{GraphBase, WeightedGraph};
use proconio::input;
use shortest_path::{self, ShortestPath};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        e: [(usize, usize, i64); m],
    }

    let mut g = WeightedGraph::initial(n);

    for (a, b, c) in e {
        g.add_edge_directed(a, b, &c);
    }

    let res = g.dijkstra(s);
    match res.dist[t] {
        Some(_) => {
            let mut ans = vec![];
            let mut now = t;
            while let Some(v) = res.prv[now] {
                ans.push((v, now));
                now = v;
            }
            println!("{} {}", res.dist[t].unwrap(), ans.len());
            for &(a, b) in ans.iter().rev() {
                println!("{} {}", a, b);
            }
        }
        None => {
            println!("-1");
        }
    }
}
