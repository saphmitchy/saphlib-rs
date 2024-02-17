use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Add;

use zero::Zero;
use graph_base;

pub trait ShortestPath<Weight>
where
    Self: graph_base::GraphBase,
    Weight: Clone + Add<Output = Weight> + Ord + Zero,
{
    fn get_weights<'a>(&'a self, e: usize) -> Vec<(usize, &'a Weight)>;

    fn dijkstra(&self, s: usize) -> Vec<Option<Weight>> {
        let mut res = vec![None; self.vertex_count()];
        let mut h = BinaryHeap::new();
        h.push((Reverse(Weight::zero()), s));
        while let Some((d, v)) = h.pop() {
            if !res[v].is_none() {
                continue;
            }
            for (to, w) in self.get_weights(v) {
                if res[to].is_none() {
                    h.push((Reverse(w.clone() + d.clone().0), to));
                }
            }
            res[v] = Some(d.0);
        }
        res
    }
}

impl ShortestPath<usize> for graph_base::UnweightedGraph {
    fn get_weights<'a>(&'a self, e: usize) -> Vec<(usize, &'a usize)> {
        self.get_edges(e)
            .iter()
            .map(|to| (to.clone(), &1usize))
            .collect::<Vec<_>>()
    }
}

impl<T: Clone + Add<Output = T> + Ord + Zero> ShortestPath<T> for graph_base::WeightedGraph<T> {
    fn get_weights<'a>(&'a self, e: usize) -> Vec<(usize, &'a T)> {
        self.get_edges(e)
            .iter()
            .map(|x| (x.0.clone(), &x.1))
            .collect()
    }
}
