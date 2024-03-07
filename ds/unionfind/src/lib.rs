pub struct UnionFind {
    n: usize,
    components: usize,
    parrent_size: Vec<i32>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            n: n,
            components: n,
            parrent_size: vec![-1; n],
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        if self.parrent_size[x] < 0 {
            return x;
        }
        let parent = self.parrent_size[x];
        self.parrent_size[x] = self.root(parent as usize) as i32;
        self.parrent_size[x] as usize
    }

    pub fn unite(&mut self, x: usize, y: usize) -> usize {
        assert!(x < self.n);
        assert!(y < self.n);
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y {
            return x;
        }
        self.components -= 1;
        if self.parrent_size[x] > self.parrent_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parrent_size[x] += self.parrent_size[y];
        self.parrent_size[y] = x as i32;
        x
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.n);
        assert!(y < self.n);
        self.root(x) == self.root(y)
    }

    pub fn size(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        let root = self.root(x);
        -self.parrent_size[root] as usize
    }

    pub fn components(&mut self) -> usize {
        self.components
    }

    pub fn group(&mut self) -> Vec<Vec<usize>> {
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            let root = self.root(i);
            group_size[root] += 1;
        }
        let mut res = vec![vec![]; self.n];
        for i in 0..self.n {
            res[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            let root = self.root(i);
            res[root].push(i);
        }
        res.into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(10);
        assert_eq!(uf.components(), 10);
        assert_eq!(uf.root(5), 5);
        assert_eq!(uf.size(5), 1);
        uf.unite(4, 5);
        uf.unite(3, 5);
        uf.unite(2, 7);
        uf.unite(1, 9);
        uf.unite(3, 4);
        uf.unite(0, 8);
        let tmp = uf.unite(4, 0);
        assert_eq!(uf.root(4), tmp);
        assert_eq!(uf.components(), 4);
        assert!(uf.same(3, 8));
        assert!(uf.same(1, 9));
        assert!(!uf.same(3, 9));
        assert!(!uf.same(2, 6));
        let mut g = uf.group();
        g.sort();
        assert_eq!(
            g,
            vec![vec![0, 3, 4, 5, 8], vec![1, 9], vec![2, 7], vec![6]]
        );
    }
}
