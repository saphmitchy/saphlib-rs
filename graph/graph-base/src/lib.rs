pub struct UnweightedGraph {
    pub n: usize,
    pub edges: Vec<Vec<usize>>,
}

pub struct WeightedGraph<T: Clone> {
    pub n: usize,
    pub edges: Vec<Vec<(usize, T)>>,
}

pub trait GraphBase {
    fn initial(n: usize) -> Self
    where
        Self: Sized;
    fn vertex_count(&self) -> usize;
}

macro_rules! impl_graphbase {
    ($t:ty,  "unweighted") => {
        impl GraphBase for $t {
            impl_graphbase! {@body}
        }
    };

    ($t:ty,  "weighted") => {
        impl<T: Clone> GraphBase for $t {
            impl_graphbase! {@body}
        }
    };

    (@body) => {
        fn initial(n: usize) -> Self {
            Self {
                n,
                edges: vec![vec![]; n],
            }
        }

        fn vertex_count(&self) -> usize {
            self.n
        }
    };
}

impl_graphbase! {UnweightedGraph, "unweighted"}
impl_graphbase! {WeightedGraph<T>, "weighted"}

impl UnweightedGraph {
    pub fn add_edge_directed(&mut self, from: usize, to: usize) {
        assert!(from < self.n);
        assert!(to < self.n);
        self.edges[from].push(to);
    }

    pub fn add_edge_undirected(&mut self, from: usize, to: usize) {
        assert!(from < self.n);
        assert!(to < self.n);
        self.edges[from].push(to);
        self.edges[to].push(from);
    }

    pub fn get_edges<'a>(&'a self, v: usize) -> &'a Vec<usize> {
        assert!(v < self.n);
        self.edges.get(v).unwrap()
    }
}

impl<T: Clone> WeightedGraph<T> {
    pub fn add_edge_directed(&mut self, from: usize, to: usize, w: &T) {
        assert!(from < self.n);
        assert!(to < self.n);
        self.edges[from].push((to, w.clone()));
    }

    pub fn add_edge_undirected(&mut self, from: usize, to: usize, w: &T) {
        assert!(from < self.n);
        assert!(to < self.n);
        self.edges[from].push((to, w.clone()));
        self.edges[to].push((from, w.clone()));
    }

    pub fn get_edges<'a>(&'a self, v: usize) -> &'a Vec<(usize, T)> {
        assert!(v < self.n);
        self.edges.get(v).unwrap()
    }
}
