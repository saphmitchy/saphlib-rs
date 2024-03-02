use graph_base::{self, Edge, GraphBase};

#[derive(Debug)]
pub struct Lowlink {
    ord: Vec<usize>,
    low: Vec<usize>,
    is_root: Vec<bool>,
    is_tree_edge: Vec<bool>,
    is_articulation: Vec<bool>,
}

impl Lowlink {
    const MAX: usize = usize::MAX;

    pub fn dfs(
        &mut self,
        g: &graph_base::UnweightedGraph,
        x: usize,
        from: usize,
        counter: usize,
    ) -> usize {
        let mut counter = counter + 1;
        self.ord[x] = counter;
        self.low[x] = counter;
        let mut arti_flg = false;
        for &Edge { to, id, .. } in g.get_edges(x) {
            if self.ord[to] == Self::MAX {
                self.is_tree_edge[id] = true;
                counter = self.dfs(g, to, x, counter);
                self.low[x] = self.low[x].min(self.low[to]);
                arti_flg = arti_flg || (self.ord[x] <= self.low[to]);
            } else {
                self.is_tree_edge[id] = false;
                if to != from {
                    self.low[x] = self.low[x].min(self.ord[to]);
                }
            }
        }
        if from == Self::MAX {
            self.is_articulation[x] = g
                .get_edges(x)
                .iter()
                .filter(|x| self.is_tree_edge[x.id])
                .count()
                >= 2;
        } else {
            self.is_articulation[x] = arti_flg;
        }
        counter
    }

    pub fn new(g: &graph_base::UnweightedGraph) -> Lowlink {
        let mut res = Lowlink {
            ord: vec![Self::MAX; g.vertex_count()],
            low: vec![Self::MAX; g.vertex_count()],
            is_root: vec![false; g.vertex_count()],
            is_tree_edge: vec![false; g.edge_count()],
            is_articulation: vec![false; g.vertex_count()],
        };
        let mut counter = 0;
        for i in 0..g.vertex_count() {
            if res.ord[i] == Self::MAX {
                res.is_root[i] = true;
                counter = res.dfs(g, i, Self::MAX, counter);
            }
        }
        res
    }

    pub fn is_bridge<T>(&self, e: &Edge<T>) -> bool {
        let &Edge::<T> { from, to, .. } = e;
        if self.ord[from] < self.ord[to] {
            self.ord[from] < self.low[to]
        } else {
            self.ord[to] < self.low[from]
        }
    }

    pub fn is_articulation(&self, v: usize) -> bool {
        self.is_articulation[v]
    }
}
