use graph_base::{GraphBase, UndirectedEdge, UndirectedGraph};

#[derive(Debug)]
pub struct Lowlink<'a, T: Clone> {
    g: &'a UndirectedGraph<T>,
    ord: Vec<usize>,
    low: Vec<usize>,
    is_root: Vec<bool>,
    is_tree_edge: Vec<bool>,
    is_articulation: Vec<bool>,
}

impl<'a, T: Clone> Lowlink<'a, T> {
    const MAX: usize = usize::MAX;

    pub fn dfs(
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

    pub fn new(g: &'a graph_base::UndirectedGraph<T>) -> Lowlink<'a, T> {
        let mut res = Lowlink {
            g: &g,
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

    pub fn is_bridge(&self, e: &UndirectedEdge<T>) -> bool {
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

    /// return (vertex, edge)
    pub fn biconnected_components(&self) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
        let mut res0 = vec![];
        let mut res1 = vec![];
        for i in 0..self.g.vertex_count() {
            let mut has_child = true;
            if self.is_root[i] {
                for e in self.g.get_edges(i) {
                    if self.is_tree_edge[e.id()] {
                        has_child = false;
                        res0.push(vec![i]);
                        res1.push(vec![e.id()]);
                        let k = res0.len() - 1;
                        self.biconnected_components_inner(e.another_side(i), &mut res0, &mut res1, k);
                    }
                }
                if has_child {
                    res0.push(vec![i]);
                    res1.push(vec![]);
                }
            }
        }
        (res0, res1)
    }

    fn biconnected_components_inner(
        &self,
        now: usize,
        res0: &mut Vec<Vec<usize>>,
        res1: &mut Vec<Vec<usize>>,
        idx: usize,
    ) -> () {
        res0[idx].push(now);
        for e in self.g.get_edges(now) {
            let to = e.another_side(now);
            let id = e.id();
            if self.is_tree_edge[id] {
                if self.ord[now] <= self.low[to] {
                    res0.push(vec![now]);
                    res1.push(vec![id]);
                    self.biconnected_components_inner(to, res0, res1, res0.len() - 1);
                } else if self.ord[now] < self.ord[to] {
                    res1[idx].push(id);
                    self.biconnected_components_inner(to, res0, res1, idx);
                }
            } else if self.ord[now] > self.ord[to] {
                res1[idx].push(id);
            }
        }
    }
}
