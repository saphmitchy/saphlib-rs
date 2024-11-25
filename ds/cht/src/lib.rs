use std::{collections::VecDeque, marker::PhantomData};

/// line: y = ax + b
#[derive(Debug, Clone, Copy)]
struct Line {
    a: i64,
    b: i64,
}

impl Line {
    fn new(a: i64, b: i64) -> Line {
        Line { a, b }
    }

    fn get(&self, x: i64) -> i64 {
        self.a * x + self.b
    }
}

pub trait CalcMarker {
    fn negate_if_max(x: i64) -> i64;
}

#[derive(Debug, Clone, Copy)]
pub struct CalcMax;
impl CalcMarker for CalcMax {
    fn negate_if_max(x: i64) -> i64 {
        -x
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CalcMin;
impl CalcMarker for CalcMin {
    fn negate_if_max(x: i64) -> i64 {
        x
    }
}

/// # Convex Hull Trick
/// ```
/// use cht::*;
/// let mut cht = Cht::<CalcMin>::new();
///
/// cht.add_line_right(10, -5);
/// cht.add_line_right(5, 8);
/// cht.add_line_right(2, -1);
/// cht.add_line_right(-1, 4);
///
/// assert_eq!(cht.get(0), -5);
/// assert_eq!(cht.get(4), 0);
/// assert_eq!(cht.get(1), 1);
/// ```
#[derive(Debug, Clone)]
pub struct Cht<C: CalcMarker> {
    q: VecDeque<Line>,
    __marker: PhantomData<C>,
}

impl<C: CalcMarker> Cht<C> {
    pub fn new() -> Cht<C> {
        Cht {
            q: VecDeque::new(),
            __marker: PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.q.is_empty()
    }

    /// if l2 can be deleted in l1, l2 and l3.
    fn judge(l1: &Line, l2: &Line, l3: &Line) -> bool {
        let a1 = (l2.a - l1.a) as i128;
        let b1 = (l2.b - l1.b) as i128;
        let a2 = (l3.a - l2.a) as i128;
        let b2 = (l3.b - l2.b) as i128;
        a2 * b1 <= a1 * b2
    }

    fn add_line_left_inner(&mut self, l: Line) {
        assert!(self.q.front().is_none_or(|x| x.a <= l.a));
        if self.q.front().is_some_and(|x| x.a == l.a) {
            if self.q.front().unwrap().b <= l.b {
                return;
            }
            self.q.pop_front();
        }
        while 2 <= self.q.len() {
            let l2 = self.q[0];
            let l3 = self.q[1];
            if !Self::judge(&l, &l2, &l3) {
                break;
            }
            self.q.pop_front();
        }
        self.q.push_front(l);
    }

    /// add line ax + b to left most side.
    /// if calculating min, slope must be monotonically increasing.
    /// if calculating max, slope must be monotonically decreasing.
    pub fn add_line_left(&mut self, a: i64, b: i64) {
        self.add_line_left_inner(Line::new(C::negate_if_max(a), C::negate_if_max(b)));
    }

    fn add_line_right_inner(&mut self, l: Line) {
        assert!(self.q.back().is_none_or(|x| l.a <= x.a));
        if self.q.back().is_some_and(|x| x.a == l.a) {
            if self.q.back().unwrap().b <= l.b {
                return;
            }
            self.q.pop_back();
        }
        while 2 <= self.q.len() {
            let back = self.q.len() - 1;
            let l1 = self.q[back - 1];
            let l2 = self.q[back];
            if !Self::judge(&l1, &l2, &l) {
                break;
            }
            self.q.pop_back();
        }
        self.q.push_back(l);
    }

    /// add line ax + b to right most side.
    /// if calculating min, slope must be monotonically decreasing.
    /// if calculating max, slope must be monotonically increasing.
    pub fn add_line_right(&mut self, a: i64, b: i64) {
        self.add_line_right_inner(Line::new(C::negate_if_max(a), C::negate_if_max(b)));
    }

    pub fn get(&self, x: i64) -> Option<i64> {
        if self.is_empty() {
            return None;
        }
        let mut l = 0;
        let mut r = self.q.len();
        while r - l > 1 {
            let m = (l + r) / 2;
            if self.q[m - 1].get(x) >= self.q[m].get(x) {
                l = m;
            } else {
                r = m;
            }
        }
        let res = self.q[l].get(x);
        return Some(C::negate_if_max(res));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cht_min() {
        let mut cht = Cht::<CalcMin>::new();

        cht.add_line_right(10, -5);
        cht.add_line_right(5, 8);
        cht.add_line_right(2, -1);
        cht.add_line_right(-1, 4);

        assert_eq!(cht.get(0), Some(-5));
        assert_eq!(cht.get(4), Some(0));
        assert_eq!(cht.get(1), Some(1));
    }
}
