use graph_base;

#[derive(Debug)]
pub struct Lowlink {
    ord: Vec<usize>,
    low: Vec<usize>,
    is_root: Vec<bool>,
    is_tree: Vec<Vec<bool>>,
}

impl Lowlink {
    const MAX: usize = usize::MAX;

    pub fn dfs(&mut self, g: &graph_base::UnweightedGraph, x: usize, from: usize, counter: usize) -> usize {
        let mut counter = counter + 1;
        self.ord[x] = counter;
        self.low[x] = counter;
        for i in g.edges[x].clone() {
            if self.ord[i] == Self::MAX {
                self.is_tree[x].push(true);
                counter = self.dfs(g, i, x, counter);
                if i != from {
                    self.low[x] = self.low[x].min(self.low[i]);
                }
            } else {
                self.is_tree[x].push(false);
                self.low[x] = self.low[x].min(self.ord[i]);
            }
        }
        counter
    }

    pub fn new(g: &graph_base::UnweightedGraph) -> Lowlink {
        let mut res = Lowlink {
            ord: vec![Self::MAX; g.n],
            low: vec![Self::MAX; g.n],
            is_root: vec![false; g.n],
            is_tree: vec![vec![]; g.n],
        };
        let mut counter = 0;
        for i in 0..g.n {
            if res.ord[i] == Self::MAX {
                res.is_root[i] = true;
                counter = res.dfs(g, i, Self::MAX, counter);
            }
        }
        res
    }
}
