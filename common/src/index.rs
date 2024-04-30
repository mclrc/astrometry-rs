use kd_tree::KdTree;
use serde::{Deserialize, Serialize};

use crate::quad::Quad;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct IndexStar {
    designation: String,
    position: [f64; 2],
}

#[derive(Serialize, Deserialize)]
pub struct Index {
    nside: u32,
    quad_index: KdTree<([f64; 4], Quad<IndexStar>)>,
    position_index: KdTree<([f64; 2], IndexStar)>,
}

impl Index {
    pub fn new(
        nside: u32,
        quads: impl Iterator<Item = Quad<IndexStar>>,
        stars: impl Iterator<Item = IndexStar>,
    ) -> Self {
        let quad_points = quads.map(|q| (q.ghash(), q)).collect::<Vec<_>>();
        let position_points = stars.map(|s| (s.position, s)).collect::<Vec<_>>();

        Self {
            nside,
            quad_index: KdTree::build_by_ordered_float(quad_points),
            position_index: KdTree::build_by_ordered_float(position_points),
        }
    }

    pub fn nside(&self) -> u32 {
        self.nside
    }

    pub fn quad_index(&self) -> &KdTree<([f64; 4], Quad<IndexStar>)> {
        &self.quad_index
    }

    pub fn position_index(&self) -> &KdTree<([f64; 2], IndexStar)> {
        &self.position_index
    }
}
