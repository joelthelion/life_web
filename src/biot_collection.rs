use macroquad::prelude::*;
use oorandom::Rand32;
use crate::biot::{Biot, TreePoint};
use rstar::RTree;

pub struct BiotCollection {
    biots: Vec<Biot>
}

impl BiotCollection {
    pub fn new(n: usize, rng: &mut Rand32) -> Self {
        let mut s = Self { biots: Vec::new() };
        for _ in 0..n {
            s.biots.push(Biot::random_biot(rng));
        }
        s
    }
    pub fn step(&mut self, rng: &mut Rand32) {
        let mut new : Vec<Biot> = Vec::new();
        let tree : RTree<TreePoint> = RTree::bulk_load(
            self.biots
                .iter()
                .enumerate()
                .map(|(n,biot)|TreePoint {x:biot.pos.x as f64, y:biot.pos.y as f64, idx:n})
                .collect());
        for n in 0..(self.biots.len()) {
            let mut feed_dir : Option<Vec2> = None;
            if self.biots[n].intelligence > 0. {
                for (other, d2) in tree.nearest_neighbor_iter_with_distance_2(&[self.biots[n].pos.x as f64, self.biots[n].pos.y as f64]) {
                    if d2 as f32 > (self.biots[n].intelligence*self.biots[n].intelligence)*1600. {
                        break;
                    }
                    if self.biots[n].stronger(&self.biots[other.idx]) {
                        // println!("found a victim! {:?} -> {:?}", self.biots[n], self.biots[other.idx]);
                        feed_dir = Some(vec2(other.x as f32 -self.biots[n].pos.x+0.0001, other.y as f32 -self.biots[n].pos.y+0.0001).normalize());
                        break;
                    }
                }
            }
            let off = self.biots[n].step(&tree, feed_dir, rng);
            if let Some(offspring) = off {
                new.push(offspring);
            }
        }
        for f in tree.iter() {
            for s in tree.locate_within_distance([f.x, f.y], 50.) //FIXME 30 is hardcoded
            {
                if f.idx < s.idx { // Don't do it twice
                    Biot::interact(&mut self.biots, f.idx, s.idx);
                }
            }
        }
        self.biots.retain(|b| !b.dead());
        self.biots.append(&mut new);
    }
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
    pub fn len(&self) -> usize {
        self.biots.len()
    }
}

