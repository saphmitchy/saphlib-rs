use graph_base::{GraphBase, UndirectedEdge};

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

    pub fn dfs<T: Clone>(
        &mut self,
        g: &graph_base::UndirectedGraph<T>,
        x: usize,
        from: usize,
        counter: usize,
    ) -> usize {
        let mut counter = counter + 1;
        self.ord[x] = counter;
        self.low[x] = counter;
        let mut arti_flg = false;
        for e in g.get_edges(x) {
            let to = e.another_side(x);
            let id = e.id();
            if self.ord[to] == Self::MAX {
                self.is_tree_edge[id] = true;
                counter = self.dfs(g, to, x, counter);
                self.low[x] = self.low[x].min(self.low[to]);
                arti_flg = arti_flg || (self.ord[x] <= self.low[to]);
            } else {
                if to != from {
                    self.low[x] = self.low[x].min(self.ord[to]);
                }
            }
        }
        if from == Self::MAX {
            self.is_articulation[x] = g
                .get_edges(x)
                .filter(|x| self.is_tree_edge[x.id() as usize])
                .count()
                >= 2;
        } else {
            self.is_articulation[x] = arti_flg;
        }
        counter
    }

    pub fn new<T: Clone>(g: &graph_base::UndirectedGraph<T>) -> Lowlink {
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

    pub fn is_bridge<T: Clone>(&self, e: &UndirectedEdge<T>) -> bool {
        let (from, to) = e.side();
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
