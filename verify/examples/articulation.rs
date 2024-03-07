// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/GRL_3_A
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
        g.add_edge(s, t);
    }
    let lk = Lowlink::new(&g);
    let mut ans = (0..v)
        .into_iter()
        .filter(|x| lk.is_articulation(*x))
        .collect::<Vec<_>>();
    ans.sort();
    for i in ans {
        println!("{}", i);
    }
}
