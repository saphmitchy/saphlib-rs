use graph_base::{self, Edge, GraphBase};

#[derive(Debug)]
pub struct Lowlink {
    ord: Vec<usize>,
    low: Vec<usize>,
    is_root: Vec<bool>,
    is_tree_edge: Vec<bool>,
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
        for &Edge { to, id, .. } in g.get_edges(x) {
            if self.ord[to] == Self::MAX {
                self.is_tree_edge[id] = true;
                counter = self.dfs(g, to, x, counter);
                if to != from {
                    self.low[x] = self.low[x].min(self.low[to]);
                }
            } else {
                self.is_tree_edge[id] = false;
                self.low[x] = self.low[x].min(self.ord[to]);
            }
        }
        counter
    }

    pub fn new(g: &graph_base::UnweightedGraph) -> Lowlink {
        let mut res = Lowlink {
            ord: vec![Self::MAX; g.vertex_count()],
            low: vec![Self::MAX; g.vertex_count()],
            is_root: vec![false; g.vertex_count()],
            is_tree_edge: vec![false; g.edge_count()],
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
}
