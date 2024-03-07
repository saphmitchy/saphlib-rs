// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/3/GRL_3_B
use proconio::input;

use graph_base::{GraphBase, UndirectedGraph};
use lowlink::Lowlink;

fn main() {
    input! {
        v: usize,
        e: usize,
        edge: [(usize, usize); e],
    }

    let mut g = UndirectedGraph::new(v);
    for (s, t) in edge {
        g.add_edge(s.min(t), t.max(s));
    }
    let lk = Lowlink::new(&g);
    let mut ans = (0..e)
        .into_iter()
        .filter_map(|x| {
            let e = g.edge(x);
            if lk.is_bridge(e) {
                Some(e.side())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    ans.sort();
    for (s, t) in ans {
        println!("{} {}", s, t);
    }
}
