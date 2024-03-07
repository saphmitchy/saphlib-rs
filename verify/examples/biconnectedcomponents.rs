// verification-helper: PROBLEM https://judge.yosupo.jp/problem/biconnected_components

use graph_base::{GraphBase, UndirectedGraph};
use lowlink::Lowlink;
use proconio::input;

fn run(n: usize, v: &Vec<(usize, usize)>) -> bool {
    let mut g = UndirectedGraph::new(n);
    for (a, b) in v {
        g.add_edge(*a, *b);
    }
    let lk = Lowlink::new(&g);
    let c = lk.biconnected_components();
    println!("{}", c.len());
    for i in &c {
        print!("{}", i.len());
        for j in i {
            print!(" {}", j);
        }
        println!();
    }
    c.len() != n - 1
}

fn main() {
    input! {
        n: usize,
        m: usize,
        v: [(usize, usize); m],
    }
    run(n, &v);
}
