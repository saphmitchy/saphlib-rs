#[derive(Clone)]
pub struct Edge<T> {
    pub to: usize,
    pub weight: T,
    pub id: usize,
}

pub struct UnweightedGraph {
    n: usize,
    edge_count: usize,
    edges: Vec<Vec<Edge<()>>>,
}

pub struct WeightedGraph<T: Clone> {
    n: usize,
    edge_count: usize,
    edges: Vec<Vec<Edge<T>>>,
}

pub trait GraphBase {
    fn initial(n: usize) -> Self
    where
        Self: Sized;
    fn vertex_count(&self) -> usize;
    fn edge_count(&self) -> usize;
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
                edge_count: 0,
                edges: vec![vec![]; n],
            }
        }

        fn vertex_count(&self) -> usize {
            self.n
        }
        fn edge_count(&self) -> usize {
            self.edge_count
        }
    };
}

impl_graphbase! {UnweightedGraph, "unweighted"}
impl_graphbase! {WeightedGraph<T>, "weighted"}

impl UnweightedGraph {
    pub fn add_edge_directed(&mut self, from: usize, to: usize) -> usize {
        assert!(from < self.n);
        assert!(to < self.n);
        self.edges[from].push(Edge {
            to,
            weight: (),
            id: self.edge_count,
        });
        let res = self.edge_count;
        self.edge_count += 1;
        res
    }

    pub fn add_edge_undirected(&mut self, from: usize, to: usize) -> (usize, usize) {
        assert!(from < self.n);
        assert!(to < self.n);
        self.edges[from].push(Edge {
            to,
            weight: (),
            id: self.edge_count,
        });
        self.edges[to].push(Edge {
            to: from,
            weight: (),
            id: self.edge_count + 1,
        });
        let res = (self.edge_count, self.edge_count + 1);
        self.edge_count += 2;
        res
    }

    pub fn get_edges<'a>(&'a self, v: usize) -> &'a Vec<Edge<()>> {
        assert!(v < self.n);
        self.edges.get(v).unwrap()
    }
}

impl<T: Clone> WeightedGraph<T> {
    pub fn add_edge_directed(&mut self, from: usize, to: usize, w: &T) -> usize {
        assert!(from < self.n);
        assert!(to < self.n);
        self.edges[from].push(Edge {
            to,
            weight: w.clone(),
            id: self.edge_count,
        });
        let res = self.edge_count;
        self.edge_count += 1;
        res
    }

    pub fn add_edge_undirected(&mut self, from: usize, to: usize, w: &T) -> (usize, usize) {
        assert!(from < self.n);
        assert!(to < self.n);
        self.edges[from].push(Edge {
            to,
            weight: w.clone(),
            id: self.edge_count,
        });
        self.edges[to].push(Edge {
            to: from,
            weight: w.clone(),
            id: self.edge_count + 1,
        });
        let res = (self.edge_count, self.edge_count + 1);
        self.edge_count += 2;
        res
    }

    pub fn get_edges<'a>(&'a self, v: usize) -> &'a Vec<Edge<T>> {
        assert!(v < self.n);
        self.edges.get(v).unwrap()
    }
}
