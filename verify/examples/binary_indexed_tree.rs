// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use binary_indexed_tree::BinaryIndexedTree;
use magma::AddMagma;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        query: [(i64, i64, i64); q],
    }

    let mut bt = BinaryIndexedTree::<AddMagma<i64>>::from_vec(&a);

    for (p, s, t) in query {
        if p == 0 {
            bt.add(s as usize, &t);
        } else {
            println!("{}", bt.range_sum((s as usize)..(t as usize)));
        }
    }
}
