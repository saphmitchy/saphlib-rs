#[derive(Clone, Debug)]
pub struct DirectedGraph<T: Clone> {
    n: u32,
    edges: Vec<Vec<u32>>,
    edge_info: Vec<DirectedEdge<T>>,
}

#[derive(Clone, Debug)]
pub struct UndirectedGraph<T: Clone> {
    n: u32,
    edges: Vec<Vec<u32>>,
    edge_info: Vec<UndirectedEdge<T>>,
}

#[derive(Clone, Debug)]
pub struct DirectedEdge<T: Clone> {
    pub from: u32,
    pub to: u32,
    pub id: u32,
    pub weight: T,
}

impl<T: Clone> DirectedEdge<T> {
    pub fn from(&self) -> usize {
        self.from as usize
    }

    pub fn to(&self) -> usize {
        self.to as usize
    }

    pub fn id(&self) -> usize {
        self.id as usize
    }

    pub fn weight(&self) -> &T {
        &self.weight
    }
}

#[derive(Clone, Debug)]
pub struct UndirectedEdge<T: Clone> {
    side: (u32, u32),
    id: u32,
    weight: T,
}

impl<T: Clone> UndirectedEdge<T> {
    pub fn side(&self) -> (usize, usize) {
        (self.side.0 as usize, self.side.1 as usize)
    }

    pub fn another_side(&self, s: usize) -> usize {
        let (a, b) = self.side;
        let a = a as usize;
        let b = b as usize;
        assert!(a == s || b == s);
        a ^ b ^ s
    }

    pub fn id(&self) -> usize {
        self.id as usize
    }

    pub fn weight(&self) -> &T {
        &self.weight
    }
}

pub trait GraphBase
where
    Self: Sized,
{
    type EdgeType;
    fn new(n: usize) -> Self;

    fn vertex_count(&self) -> usize;
    fn edge_count(&self) -> usize;
    fn get_edges(&self, v: usize) -> Edges<'_, Self>;
    fn edge(&self, id: usize) -> &Self::EdgeType;
}

pub struct Edges<'a, T>
where
    T: GraphBase,
{
    v: std::slice::Iter<'a, u32>,
    edge_info: &'a Vec<T::EdgeType>,
}

impl<'a, T> Iterator for Edges<'a, T>
where
    T: GraphBase,
{
    type Item = &'a T::EdgeType;
    fn next(&mut self) -> Option<Self::Item> {
        self.v
            .next()
            .and_then(|i| self.edge_info.get((*i) as usize))
    }
}

impl<T: Clone> GraphBase for DirectedGraph<T> {
    type EdgeType = DirectedEdge<T>;
    fn new(n: usize) -> Self
    where
        Self: Sized,
    {
        DirectedGraph {
            n: n as u32,
            edges: vec![vec![]; n],
            edge_info: vec![],
        }
    }

    fn vertex_count(&self) -> usize {
        self.n as usize
    }

    fn edge_count(&self) -> usize {
        self.edge_info.len()
    }

    fn get_edges(&self, v: usize) -> Edges<'_, Self> {
        Edges {
            v: self.edges[v].iter(),
            edge_info: &self.edge_info,
        }
    }

    fn edge<'a>(&'a self, id: usize) -> &'a Self::EdgeType {
        &self.edge_info[id]
    }
}

impl<T: Clone> GraphBase for UndirectedGraph<T> {
    type EdgeType = UndirectedEdge<T>;
    fn new(n: usize) -> Self
    where
        Self: Sized,
    {
        UndirectedGraph {
            n: n as u32,
            edges: vec![vec![]; n],
            edge_info: vec![],
        }
    }

    fn vertex_count(&self) -> usize {
        self.n as usize
    }

    fn edge_count(&self) -> usize {
        self.edge_info.len()
    }

    fn get_edges(&self, v: usize) -> Edges<'_, Self> {
        Edges {
            v: self.edges[v].iter(),
            edge_info: &self.edge_info,
        }
    }

    fn edge<'a>(&'a self, id: usize) -> &'a Self::EdgeType {
        &self.edge_info[id]
    }
}

impl DirectedGraph<()> {
    pub fn add_edge(&mut self, from: usize, to: usize) -> usize {
        self.add_weighted_edge(from, to, &())
    }
}

impl<T: Clone> DirectedGraph<T> {
    pub fn add_weighted_edge(&mut self, from: usize, to: usize, w: &T) -> usize {
        let id = self.edge_count() as u32;
        let e = DirectedEdge {
            from: from as u32,
            to: to as u32,
            weight: w.clone(),
            id,
        };
        self.edge_info.push(e);
        self.edges[from].push(id);
        id as usize
    }
}

impl UndirectedGraph<()> {
    pub fn add_edge(&mut self, from: usize, to: usize) -> usize {
        self.add_weighted_edge(from, to, &())
    }
}

impl<T: Clone> UndirectedGraph<T> {
    pub fn add_weighted_edge(&mut self, from: usize, to: usize, w: &T) -> usize {
        let id = self.edge_count() as u32;
        let from = from as u32;
        let to = to as u32;
        let e = UndirectedEdge {
            side: (from, to),
            weight: w.clone(),
            id,
        };
        self.edge_info.push(e);
        self.edges[from as usize].push(id);
        self.edges[to as usize].push(id);
        id as usize
    }
}
