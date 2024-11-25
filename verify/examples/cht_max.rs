// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/2725
use cht::*;
use proconio::input;

struct ChtWrapper {
    index: usize,
    cht: Cht<CalcMax>,
}

impl ChtWrapper {
    fn new(index: usize) -> ChtWrapper {
        ChtWrapper {
            index,
            cht: Cht::<CalcMax>::new(),
        }
    }

    fn add_line(&mut self, a: i64, b: i64) {
        if self.index % 2 == 0 {
            self.cht.add_line_right(a, b);
        } else {
            self.cht.add_line_left(-a, b);
        }
    }

    fn get(&self, x: i64) -> Option<i64> {
        if self.index % 2 == 0 {
            self.cht.get(x)
        } else {
            self.cht.get(-x)
        }
    }
}

fn main() {
    input! {
        n: usize,
        total: usize,
        mut song: [(usize, i64, i64); n],
    }
    song.sort_by(|a, b| a.2.cmp(&b.2));

    let mut dp = (0..(total + 1))
        .map(|i| ChtWrapper::new(i))
        .collect::<Vec<_>>();
    let mut ans = i64::MIN;

    for (t, p, f) in song {
        for j in ((t + 1)..=total).rev() {
            if let Some(v) = dp[j - t].get(f) {
                let now = v + p - f * f;
                ans = ans.max(now);
                dp[j].add_line(2 * f, now - f * f);
            }
        }
        dp[t].add_line(2 * f, p - f * f);
        ans = ans.max(p);
        // println!("{:?}", dp);
    }
    println!("{}", ans);
}
