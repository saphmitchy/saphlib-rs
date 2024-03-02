// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/3/GRL_3_B
use proconio::input;

use graph_base::{Edge, GraphBase, UnweightedGraph};
use lowlink::Lowlink;

fn main() {
    input! {
        v: usize,
        e: usize,
        edge: [(usize, usize); e],
    }

    let mut g = UnweightedGraph::initial(v);
    for (s, t) in edge {
        g.add_edge_undirected(s, t).0;
    }
    let lk = Lowlink::new(&g);
    let mut ans = (0..(e * 2))
        .into_iter()
        .filter_map(|x| {
            if x % 2 == 1 {
                return None;
            }
            let e = g.edge(x);
            let Edge { from, to, .. } = e;
            if lk.is_bridge(e) {
                Some((from.min(to), to.max(from)))
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
