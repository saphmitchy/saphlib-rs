use std::ops::{Bound, RangeBounds};

use magma;
use monoid;

pub struct BinaryIndexedTree<T>
where
    T: monoid::Monoid,
    <T as magma::Magma>::S: Clone,
{
    data: Vec<T::S>,
    size: usize,
}

impl<T> BinaryIndexedTree<T>
where
    T: monoid::Monoid + magma::Commutative,
    <T as magma::Magma>::S: Clone,
{
    pub fn new(n: usize) -> BinaryIndexedTree<T> {
        BinaryIndexedTree {
            data: vec![T::identity(); n + 1],
            size: n,
        }
    }

    pub fn from_vec(v: &Vec<<T as magma::Magma>::S>) -> BinaryIndexedTree<T> {
        let size = v.len();
        let mut data = vec![T::identity(); size + 1];
        for i in 1..=size {
            data[i] = T::binary_operation(data[i].clone(), v[i - 1].clone());
            let nxt = (i & !(i - 1)) + i;
            if nxt <= size {
                data[nxt] = T::binary_operation(data[nxt].clone(), data[i].clone());
            }
        }
        BinaryIndexedTree { data, size }
    }

    pub fn sum(&self, index: usize) -> <T as magma::Magma>::S {
        assert!(index <= self.size);
        let mut index = index;
        let mut res = T::identity();
        while 0 < index {
            res = T::binary_operation(res, self.data[index].clone());
            index -= index & !(index - 1);
        }
        res
    }

    pub fn add(&mut self, index: usize, val: &<T as magma::Magma>::S) -> () {
        assert!(index < self.size);
        let mut index = index + 1;
        while index <= self.size {
            self.data[index] = T::binary_operation(val.clone(), self.data[index].clone());
            index += index & !(index - 1);
        }
    }
}

impl<T> BinaryIndexedTree<T>
where
    T: group::Group + magma::Commutative,
    <T as magma::Magma>::S: Clone,
{
    pub fn range_sum<R>(&self, range: R) -> <T as magma::Magma>::S
    where
        R: RangeBounds<usize>,
    {
        let upper = match range.end_bound() {
            Bound::Included(x) => *x + 1,
            Bound::Excluded(x) => *x,
            Bound::Unbounded => self.size,
        };
        let lower = match range.start_bound() {
            Bound::Included(x) => *x,
            Bound::Excluded(x) => *x + 1,
            Bound::Unbounded => 0,
        };
        assert!(upper <= self.size);
        assert!(lower < self.size);
        T::binary_operation(self.sum(upper), T::inverse(self.sum(lower)))
    }
}

#[cfg(test)]
mod test {
    use std::ops::Bound;

    use magma::AddMagma;

    use crate::BinaryIndexedTree;

    #[test]
    fn test() {
        let v = vec![3, 1, 4, 1, 5, 9, 2];
        let mut bit = BinaryIndexedTree::<AddMagma<i32>>::from_vec(&v);
        assert_eq!(
            (0..=7).map(|i| bit.sum(i)).collect::<Vec<_>>(),
            vec![0, 3, 4, 8, 9, 14, 23, 25]
        );
        bit.add(3, &1);
        bit.add(4, &9);
        assert_eq!(
            (0..=7).map(|i| bit.sum(i)).collect::<Vec<_>>(),
            vec![0, 3, 4, 8, 10, 24, 33, 35]
        );
        assert_eq!(bit.range_sum(0..7), 35);
        assert_eq!(bit.range_sum(..), 35);
        assert_eq!(bit.range_sum(2..=3), 6);
        assert_eq!(bit.range_sum(3..), 27);
        assert_eq!(bit.range_sum((Bound::Excluded(3), Bound::Excluded(5))), 14);
    }
}
