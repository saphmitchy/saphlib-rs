use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Add;

use graph_base;
use zero::Zero;

#[derive(Debug, Clone)]
pub struct SSSPResult<Weight>
where
    Weight: Clone + Add<Output = Weight> + Ord + Zero,
{
    pub size: usize,
    pub source: usize,
    pub prv: Vec<Option<usize>>,
    pub dist: Vec<Option<Weight>>,
}

pub trait ShortestPath<Weight>
where
    Self: graph_base::GraphBase,
    Weight: Clone + Add<Output = Weight> + Ord + Zero,
{
    fn get_weights<'a>(&'a self, e: usize) -> Vec<(usize, &'a Weight)>;

    fn dijkstra(&self, s: usize) -> SSSPResult<Weight> {
        let mut dist = vec![None; self.vertex_count()];
        let mut prv = vec![None; self.vertex_count()];
        let mut h = BinaryHeap::new();
        h.push((Reverse(Weight::zero()), None, s));
        while let Some((d, p, v)) = h.pop() {
            if !dist[v].is_none() {
                continue;
            }
            for (to, w) in self.get_weights(v) {
                if dist[to].is_none() {
                    h.push((Reverse(w.clone() + d.clone().0), Some(v), to));
                }
            }
            dist[v] = Some(d.0);
            prv[v] = p;
        }

        SSSPResult {
            size: self.vertex_count(),
            source: s,
            dist,
            prv,
        }
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
