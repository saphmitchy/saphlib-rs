use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, Range},
};

fn monotone_minima_inner<T, F>(h: Range<usize>, w: Range<usize>, ans: &mut Vec<T>, f: &F) -> ()
where
    T: Ord,
    F: Fn(usize, usize) -> T,
{
    if h.start == h.end {
        return;
    }
    let mid = (h.start + h.end) / 2;
    let mut idx = w.start;
    let mut val = f(mid, w.start);
    for i in (w.start + 1)..w.end {
        let tmp = f(mid, i);
        if tmp < val {
            val = tmp;
            idx = i;
        }
    }
    monotone_minima_inner(h.start..mid, w.start..(idx + 1), ans, f);
    ans.push(val);
    monotone_minima_inner((mid + 1)..h.end, idx..w.end, ans, f);
}

/// 各h.start <= i < h.endに対して res\[i\] = min_{w.start <= j < w.end} f(i, j) を返す．
/// ただし，i < i'に対して argmin_{w.start <= j < w.end} f(i, j) <= min_{w.start <= j < w.end} f(i', j)
pub fn monotone_minima<T, F>(h: Range<usize>, w: Range<usize>, f: &F) -> Vec<T>
where
    T: Ord,
    F: Fn(usize, usize) -> T,
{
    let mut ans = Vec::with_capacity(h.end);
    monotone_minima_inner(h, w, &mut ans, f);
    ans
}

#[derive(PartialEq, Eq)]
enum MyOptional<T: Eq> {
    Some(T),
    None,
}

impl<T: Eq + Debug> Debug for MyOptional<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyOptional::Some(x) => {
                write!(f, "Some({:?})", x)
            }
            MyOptional::None => {
                write!(f, "None")
            }
        }
    }
}

impl<T: Eq> MyOptional<T> {
    fn unwrap(self) -> T {
        if let MyOptional::Some(x) = self {
            return x;
        }
        panic!();
    }
}

impl<T: Ord> PartialOrd for MyOptional<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for MyOptional<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let MyOptional::Some(lhs) = self {
            if let MyOptional::Some(rhs) = other {
                lhs.cmp(rhs)
            } else {
                Ordering::Less
            }
        } else {
            if let MyOptional::Some(_) = other {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }
}

pub fn minplus_convolve<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<T>
where
    T: Add<Output = T> + Ord + Clone + Copy,
{
    let m = a.len();
    let f = |i: usize, j: usize| {
        if j <= i && i < m + j {
            MyOptional::Some(a[i - j] + b[j])
        } else {
            MyOptional::None
        }
    };
    let res = monotone_minima(0..(a.len() + b.len() - 1), 0..b.len(), &f);
    res.into_iter().map(|x| x.unwrap()).collect()
}

#[cfg(test)]
mod test {
    use crate::{minplus_convolve, monotone_minima};

    #[test]
    fn test1() {
        let v = [[0, 5, 3], [-1, 2, 4], [4, 5, 2]];
        let f = |i: usize, j: usize| v[i][j];
        assert_eq!(monotone_minima(0..3, 0..3, &f), vec![0, -1, 2]);
    }
    #[test]
    fn test2() {
        let a = vec![3, 1, 0, 3];
        let b = vec![5, 1, 3, 3, 2];
        let c = minplus_convolve(&a, &b);
        assert_eq!(c, vec![8, 4, 2, 1, 3, 3, 2, 5]);
    }
}
