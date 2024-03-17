// verification-helper: PROBLEM https://judge.yosupo.jp/problem/min_plus_convolution_convex_arbitrary
use monotone_minima::minplus_convolve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        b: [i64; m],
    };
    let ans = minplus_convolve(&a, &b);
    for i in 0..ans.len() {
        if i != ans.len() - 1 {
            print!("{}\n", ans[i])
        } else {
            print!("{} ", ans[i])
        }
    }
}
