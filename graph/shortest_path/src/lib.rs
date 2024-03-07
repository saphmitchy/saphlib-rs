use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Add;

use graph_base::{self, GraphBase};
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
{
    fn get_weights(&self, e: usize) -> Vec<(usize, Weight)>;

    fn dijkstra(&self, s: usize) -> SSSPResult<Weight>
    where
        Weight: Clone + Add<Output = Weight> + Ord + Zero,
    {
        self.dijkstra_inner(s)
    }
}

trait ShortestPathInnter<Weight>
where
    Weight: Clone + Add<Output = Weight> + Ord + Zero,
{
    fn dijkstra_inner(&self, s: usize) -> SSSPResult<Weight>;
}

impl<T, Weight> ShortestPathInnter<Weight> for T
where
    Weight: Clone + Add<Output = Weight> + Ord + Zero,
    T: ShortestPath<Weight>,
{
    fn dijkstra_inner(&self, s: usize) -> SSSPResult<Weight> {
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

impl<T: Clone> ShortestPath<T> for graph_base::DirectedGraph<T> {
    fn get_weights(&self, e: usize) -> Vec<(usize, T)> {
        self.get_edges(e)
            .map(|x| (x.to as usize, x.weight.clone()))
            .collect()
    }
}

impl<T: Clone> ShortestPath<T> for graph_base::UndirectedGraph<T> {
    fn get_weights(&self, e: usize) -> Vec<(usize, T)> {
        self.get_edges(e)
            .map(|x| (x.another_side(e), x.weight().clone()))
            .collect()
    }
}
