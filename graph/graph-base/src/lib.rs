#[derive(Clone)]
pub struct Edge<T> {
    pub from: usize,
    pub to: usize,
    pub weight: T,
    pub id: usize,
}

struct EdgeInfo {
    from: u32,
    idx: u32,
}

pub struct UnweightedGraph {
    n: usize,
    edges: Vec<Vec<Edge<()>>>,
    edge_info: Vec<EdgeInfo>,
}

pub struct WeightedGraph<T: Clone> {
    n: usize,
    edges: Vec<Vec<Edge<T>>>,
    edge_info: Vec<EdgeInfo>,
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
                edges: vec![vec![]; n],
                edge_info: vec![],
            }
        }

        #[inline]
        fn vertex_count(&self) -> usize {
            self.n
        }

        #[inline]
        fn edge_count(&self) -> usize {
            self.edge_info.len()
        }
    };
}

impl_graphbase! {UnweightedGraph, "unweighted"}
impl_graphbase! {WeightedGraph<T>, "weighted"}

impl UnweightedGraph {
    pub fn add_edge_directed(&mut self, from: usize, to: usize) -> usize {
        assert!(from < self.n);
        assert!(to < self.n);
        let edge = Edge {
            from,
            to,
            weight: (),
            id: self.edge_count(),
        };
        self.edges[from].push(edge);
        let res = self.edge_count();
        self.edge_info.push(EdgeInfo {
            from: from as u32,
            idx: (self.edges[from].len() - 1) as u32,
        });
        res
    }

    pub fn add_edge_undirected(&mut self, from: usize, to: usize) -> (usize, usize) {
        let res0 = self.add_edge_directed(from, to);
        let res1 = self.add_edge_directed(to, from);
        (res0, res1)
    }

    #[inline]
    pub fn get_edges<'a>(&'a self, v: usize) -> &'a Vec<Edge<()>> {
        assert!(v < self.n);
        self.edges.get(v).unwrap()
    }

    #[inline]
    pub fn edge<'a>(&'a self, id: usize) -> &'a Edge<()> {
        assert!(id < self.edge_count());
        let EdgeInfo { from, idx } = self.edge_info[id];
        &self.edges[from as usize][idx as usize]
    }
}

impl<T: Clone> WeightedGraph<T> {
    pub fn add_edge_directed(&mut self, from: usize, to: usize, w: &T) -> usize {
        assert!(from < self.n);
        assert!(to < self.n);
        let edge = Edge {
            from,
            to,
            weight: w.clone(),
            id: self.edge_count(),
        };
        self.edges[from].push(edge);
        let res = self.edge_count();
        self.edge_info.push(EdgeInfo {
            from: from as u32,
            idx: (self.edges[from].len() - 1) as u32,
        });
        res
    }

    pub fn add_edge_undirected(&mut self, from: usize, to: usize, w: &T) -> (usize, usize) {
        let res0 = self.add_edge_directed(from, to, w);
        let res1 = self.add_edge_directed(to, from, w);
        (res0, res1)
    }

    #[inline]
    pub fn get_edges<'a>(&'a self, v: usize) -> &'a Vec<Edge<T>> {
        assert!(v < self.n);
        self.edges.get(v).unwrap()
    }

    #[inline]
    pub fn edge<'a>(&'a self, id: usize) -> &'a Edge<T> {
        assert!(id < self.edge_count());
        let EdgeInfo { from, idx } = self.edge_info[id];
        &self.edges[from as usize][idx as usize]
    }
}
