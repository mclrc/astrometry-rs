use kdtree::KdTree;
use serde::{Deserialize, Serialize};

use crate::Quad;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct IndexStar {
    designation: String,
    position: [f64; 2],
}

#[derive(Serialize, Deserialize)]
pub struct Index {
    nside: u32,
    quad_index: KdTree<f64, Quad<IndexStar>, [f64; 4]>,
    position_index: KdTree<f64, IndexStar, [f64; 2]>,
}

impl Index {
    pub fn new(
        nside: u32,
        quad_index: KdTree<f64, Quad<IndexStar>, [f64; 4]>,
        position_index: KdTree<f64, IndexStar, [f64; 2]>,
    ) -> Self {
        Self {
            nside,
            quad_index,
            position_index,
        }
    }

    pub fn nside(&self) -> u32 {
        self.nside
    }

    pub fn quad_index(&self) -> &KdTree<f64, Quad<IndexStar>, [f64; 4]> {
        &self.quad_index
    }

    pub fn position_index(&self) -> &KdTree<f64, IndexStar, [f64; 2]> {
        &self.position_index
    }
}
