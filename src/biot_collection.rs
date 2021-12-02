use macroquad::prelude::*;
use crate::biot::{Biot, TreePoint};
use rstar::RTree;
use std::collections::HashSet;

/// A collection of biots. Responsible for handling interactions between biots
pub struct BiotCollection {
    biots: Vec<Biot>
}

impl BiotCollection {
    /// Create n random biots
    pub fn new(n: usize) -> Self {
        let mut s = Self { biots: Vec::new() };
        for _ in 0..n {
            s.biots.push(Biot::random_biot());
        }
        s
    }
    /// Compute one step of the simulation.
    pub fn step(&mut self) {
        let mut new : Vec<Biot> = Vec::new();
        // R-star datastructure used for quickly locating neighbors
        let tree : RTree<TreePoint> = RTree::bulk_load(
            self.biots
                .iter()
                .enumerate()
                .map(|(n,biot)|TreePoint {x:biot.pos.x as f64, y:biot.pos.y as f64, idx:n})
                .collect());
        // Move and reproduce biots
        for n in 0..(self.biots.len()) {
            let mut feed_dir : Option<Vec2> = None;
            if self.biots[n].intelligence > 0. {
                for (other, d2) in tree.nearest_neighbor_iter_with_distance_2(&[self.biots[n].pos.x as f64, self.biots[n].pos.y as f64]) {
                    if other.idx == n {
                        // Filter the biot itself out of the query.
                        continue;
                    }
                    if d2 as f32 > (self.biots[n].intelligence*self.biots[n].intelligence)*1600. {
                        break;
                    }
                    if self.biots[n].stronger(&self.biots[other.idx]) {
                        feed_dir = Some(vec2(other.x as f32 -self.biots[n].pos.x, other.y as f32 -self.biots[n].pos.y).normalize());
                        break;
                    }
                }
            }
            let off = self.biots[n].step(&tree, feed_dir);
            if let Some(offspring) = off {
                new.push(offspring);
            }
        }
        // Compute biot interactions
        let mut visited : HashSet<usize> = HashSet::new();
        for f in tree.iter() {
            visited.insert(f.idx);
            for s in tree.locate_within_distance([f.x, f.y], 50.) //FIXME 30 is hardcoded
            {
                if ! visited.contains(&s.idx) { // Don't do it twice
                    Biot::interact(&mut self.biots, f.idx, s.idx);
                }
            }
        }
        // Remove dead biots and add the new ones to the collection
        self.biots.retain(|b| !b.dead());
        self.biots.append(&mut new);
    }
    /// Display the biot collection
    pub fn draw(&self) {
        for biot in self.biots.iter() {
            if biot.intelligence>0. {
                let size = 14.*(biot.photosynthesis+biot.attack+biot.defense+biot.motion);
                draw_rectangle(biot.pos.x-size/2.,biot.pos.y-size/2., size, size, GREEN);
            }
            draw_circle(biot.pos.x,biot.pos.y, 7.*(biot.photosynthesis+biot.attack+biot.defense+biot.motion), GREEN);
            draw_circle(biot.pos.x,biot.pos.y, 7.*(biot.attack+biot.defense+biot.motion), RED);
            draw_circle(biot.pos.x,biot.pos.y, 7.*(biot.defense+biot.motion), DARKBLUE);
            draw_circle(biot.pos.x,biot.pos.y, 7.*(biot.motion), BLUE);

        }
    }
    /// The number of biots currently in our collection
    pub fn len(&self) -> usize {
        self.biots.len()
    }
}

