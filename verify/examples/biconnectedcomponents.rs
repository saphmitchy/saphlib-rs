// verification-helper: PROBLEM https://judge.yosupo.jp/problem/biconnected_components

use std::collections::HashSet;

use graph_base::{GraphBase, UndirectedGraph};
use lowlink::Lowlink;
use proconio::input;

fn run(n: usize, v: &Vec<(usize, usize)>) -> bool {
    let mut g = UndirectedGraph::new(n);
    for (a, b) in v {
        g.add_edge(*a, *b);
    }
    let lk = Lowlink::new(&g);
    let (c, d) = lk.biconnected_components();
    // println!("{:?}, {:?}", c, d);
    println!("{}", c.len());
    for (i, k) in c.iter().zip(&d) {
        print!("{}", i.len());
        for j in i {
            print!(" {}", j);
        }
        let p = i.iter().collect::<HashSet<_>>();
        for j in k {
            let (s, t) = g.edge(*j).side();
            assert!(p.contains(&s));
            assert!(p.contains(&t));
        }
        println!();
    }
    assert!(d.iter().map(|p| p.iter()).flatten().collect::<HashSet<_>>().len() == v.len());
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
