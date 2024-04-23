use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};
use std::f64::consts::SQRT_2;
use std::fmt::Debug;

type GHash = [f64; 4];

#[derive(Debug)]
pub struct Quad<Star = (), Meta = ()> {
    stars: [Star; 4],
    ghash: GHash,
    meta: Meta,
}

impl<Star: Debug + Clone, Meta> Quad<Star, Meta> {
    pub fn new(stars: [((f64, f64), Star); 4], meta: Meta) -> Option<Self> {
        let star_positions = [stars[0].0, stars[1].0, stars[2].0, stars[3].0];

        if let Some((ghash, arrangement)) = Self::compute_ghash(&star_positions) {
            let stars = [
                stars[arrangement[0]].1.clone(),
                stars[arrangement[1]].1.clone(),
                stars[arrangement[2]].1.clone(),
                stars[arrangement[3]].1.clone(),
            ];

            Some(Self { ghash, meta, stars })
        } else {
            None
        }
    }

    /// Compute the geometric hash of the given set of stars.
    ///
    /// The geometric hash works as follows:
    /// A becomes (0, 0)
    /// B becomes (1, 1)
    /// The positions of C and D will be expressed in terms of the coordinate system
    /// defined by A and B.
    ///
    /// The geometric hash is the tuple (C.x, C.y, D.x, D.y)
    ///
    /// To make sure the same arrangement of stars results in the same picks
    /// for A, B, C and D, and thus the same hash, independent of the order
    /// they are passed in, we pick the arrangement that satisfies the following:
    /// 1. A and B are the stars with the largest distance between them
    /// 2. C and D lie within the circle that has A and B as its diameter
    /// 3. C.x <= D.x
    /// 4. C.x + D.x <= 1
    ///
    /// Not every set of 4 stars will have a valid arrangement that satisfies
    /// these invariants. In that case, we return None.
    fn compute_ghash(stars: &[(f64, f64); 4]) -> Option<(GHash, [usize; 4])> {
        // Find the two stars with the largest distance between them
        let (mut a_idx, mut b_idx) = (0..4)
            .tuple_combinations()
            .max_by(|&(i, j), &(k, l)| {
                let dist_ij = Vector2::new(stars[i].0 - stars[j].0, stars[i].1 - stars[j].1).norm();
                let dist_kl = Vector2::new(stars[k].0 - stars[l].0, stars[k].1 - stars[l].1).norm();
                dist_ij.partial_cmp(&dist_kl).unwrap()
            })
            .unwrap();

        // Find the two stars that are not A or B
        let mut c_idx = 0;
        while c_idx == a_idx || c_idx == b_idx {
            c_idx += 1;
        }

        let mut d_idx = 0;
        while d_idx == a_idx || d_idx == b_idx || c_idx == d_idx {
            d_idx += 1;
        }

        let a = Vector2::new(stars[a_idx].0, stars[a_idx].1);
        let b = Vector2::new(stars[b_idx].0, stars[b_idx].1) - a;
        let c = Vector2::new(stars[c_idx].0, stars[c_idx].1) - a;
        let d = Vector2::new(stars[d_idx].0, stars[d_idx].1) - a;

        // New coordinate system
        let r = SQRT_2 / 2f64;
        let xaxis = Matrix2::new(r, r, -r, r) * b / SQRT_2;
        let yaxis = Vector2::new(-xaxis[1], xaxis[0]);

        let basis_matrix = Matrix2::from_columns(&[xaxis, yaxis]);

        // Express C and D in terms of the new basis
        let mut c = basis_matrix.lu().solve(&c).unwrap();
        let mut d = basis_matrix.lu().solve(&d).unwrap();

        // Invariant 2
        let mid = Vector2::new(0.5, 0.5);
        if (c - mid).norm() > r || (d - mid).norm() > r {
            return None;
        }

        // Invariant 3
        if c[0] + d[0] > 1.0 {
            // Fix by flipping the coordinate axes (swapping A and B)
            c = Vector2::new(1f64 - c[0], 1f64 - c[1]);
            d = Vector2::new(1f64 - d[0], 1f64 - d[1]);
            (a_idx, b_idx) = (b_idx, a_idx);
        }

        // Invariant 4
        if c[0] > d[0] {
            // Fix by switching C and D
            (c, d) = (d, c);
            (c_idx, d_idx) = (d_idx, c_idx);
        }

        Some(([c[0], c[1], d[0], d[1]], [a_idx, b_idx, c_idx, d_idx]))
    }

    pub fn get_stars(&self) -> &[Star; 4] {
        &self.stars
    }

    pub fn ghash(&self) -> GHash {
        self.ghash
    }

    pub fn meta(&self) -> &Meta {
        &self.meta
    }

    pub fn assert_invariants(&self) {
        // Invariant 2
        let mid = Vector2::new(0.5, 0.5);
        assert!(
            (Vector2::new(self.ghash[0], self.ghash[1]) - mid).norm() <= SQRT_2 / 2f64
                && (Vector2::new(self.ghash[2], self.ghash[3]) - mid).norm() <= SQRT_2 / 2f64
        );
        assert!(self.ghash[0] + self.ghash[2] <= 1.0);
        assert!(self.ghash[0] <= self.ghash[2]);
    }
}

#[cfg(test)]
mod tests {
    use itertools::iproduct;
    use nalgebra::Vector4;

    use super::*;

    #[test]
    fn test_permutations() {
        let quads = [
            [(-2.44, 3.98), (3.26, -1.34), (1.9, 3.7), (-1.14, -0.46)],
            [(0.5, 1.2), (-2.3, 0.4), (2.8, -1.5), (-0.9, 1.8)],
            [(4.5, -3.2), (1.1, 1.5), (-3.3, -1.1), (0.0, 0.0)],
            [(-1.7, 2.4), (-2.9, -3.8), (3.5, 1.6), (1.2, -2.2)],
            [(2.7, 3.4), (-0.5, -0.8), (1.0, 1.2), (1.8, -3.6)],
            [(0.3, 2.7), (0.0, 0.0), (2.5, 0.9), (-2.6, 1.1)],
            [(2.2, -1.7), (-3.5, 2.8), (1.6, 3.3), (-1.2, -2.9)],
        ];

        let arrangements = [0, 1, 2, 3].into_iter().permutations(4).collect::<Vec<_>>();

        let scales = [
            // Identity
            Matrix2::identity(),
            // Scale up 2x
            Matrix2::new(2.0, 0.0, 0.0, 2.0),
            // Scale down 2x
            Matrix2::new(0.5, 0.0, 0.0, 0.5),
            // Scale up 10x
            Matrix2::new(10.0, 0.0, 0.0, 10.0),
            // Scale down 10x
            Matrix2::new(0.1, 0.0, 0.0, 0.1),
        ];

        let rotations = [
            // Identity
            Matrix2::identity(),
            // Rotate 90 degrees
            Matrix2::new(0.0, -1.0, 1.0, 0.0),
            // Rotate 180 degrees
            Matrix2::new(-1.0, 0.0, 0.0, -1.0),
            // Rotate 270 degrees
            Matrix2::new(0.0, 1.0, -1.0, 0.0),
        ];

        let translations = [
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 1.0),
            Vector2::new(-1.0, 1.0),
            Vector2::new(1000.0, -2000.0),
            Vector2::new(-1e7, 1e6),
        ];

        for stars in quads.iter() {
            let (original_ghash, _) = Quad::<()>::compute_ghash(stars).unwrap();

            for (arrangement, scale, rotation, translation) in iproduct!(
                arrangements.iter(),
                scales.iter(),
                rotations.iter(),
                translations.iter()
            ) {
                let transformed_stars: [((f64, f64), ()); 4] = arrangement
                    .iter()
                    .map(|&idx| stars[idx])
                    .map(|(x, y)| rotation * scale * Vector2::new(x, y) + translation)
                    .map(|v| ((v[0], v[1]), ()))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                let quad = Quad::new(transformed_stars, ()).unwrap();
                quad.assert_invariants();
                assert!(
                    Vector4::new(
                        quad.ghash[0] - original_ghash[0],
                        quad.ghash[1] - original_ghash[1],
                        quad.ghash[2] - original_ghash[2],
                        quad.ghash[3] - original_ghash[3]
                    )
                    .norm()
                        < 1e-7
                );
            }
        }
    }
}
