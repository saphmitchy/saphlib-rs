// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use proconio::input;
use unionfind::UnionFind;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [(usize, usize, usize); q],
    }

    let mut uf = UnionFind::new(n);

    for (t, u, v) in a {
        if t == 0 {
            uf.unite(u, v);
        } else {
            println!("{}", if uf.same(u, v) { 1 } else { 0 });
        }
    }
}
