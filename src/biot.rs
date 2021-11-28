use macroquad::math::{Vec2, vec2};
use oorandom::Rand32;
use rstar::{AABB, PointDistance, RTree, RTreeObject};

const LETTERS : &[char] = &['a','d','p','m', 'n', 'n', 'n', 'i'];
const WORLD_WIDTH : f32 = 1920.;
const WORLD_HEIGHT : f32 = 1200.;

fn modulus<T>(a:T, b:T) -> T
where T: std::ops::Rem<Output=T>+
      std::ops::Add<Output = T>+
      Copy
{
    ((a % b) + b) % b
}

#[derive(Clone, Debug)]
pub struct Biot {
    /// Status
    life: f32,
    pub pos: Vec2,
    speed: Vec2,
    age: u32,

    /// Genome
    genome: [char; 13],
    pub attack: f32,
    pub defense: f32,
    pub photosynthesis: f32,
    pub motion: f32,
    pub intelligence: f32,
}

impl Biot {
    pub fn random_biot(rng: &mut Rand32) -> Self {
        let mut genome = ['u';13];
        for letter in genome.iter_mut() {
            *letter = LETTERS[rng.rand_range(0..(LETTERS.len() as u32)) as usize];
        }
        let mut s = Self {
            life: 0.,
            pos: vec2(
                rng.rand_float()*WORLD_WIDTH,
                rng.rand_float()*WORLD_HEIGHT
                ),
            speed: vec2(0., 0.),
            age: 0,
            genome,
            attack: 0.,
            defense: 0.,
            photosynthesis: 0.,
            motion: 0.,
            intelligence: 0.,
        };
        s.set_from_genome();
        s.life = s.base_life();
        s
    }
    pub fn step(&mut self, rtree: &RTree<TreePoint>, feed_dir: Option<Vec2>, rng: &mut Rand32) -> Option<Biot> {
        let mut offspring = None;
        let adult_factor = 4.;
        if self.life >= self.base_life()*adult_factor {
            let close_by = rtree.nearest_neighbor_iter_with_distance_2(&[self.pos.x as f64, self.pos.y as f64])
                .nth(5);
            if close_by.map_or(true, |(_,d2)|d2>200.) {
                let mut off = self.clone();
                off.age = 0;
                while rng.rand_float() < 0.2 {
                    off.mutate(rng);
                }
                off.life = off.base_life();
                off.random_move(rng, 1.5);
                offspring = Some(off);
                self.life = (adult_factor-1.)* self.base_life();
            }
        }
        self.pos += self.speed;
        self.pos.x = modulus(self.pos.x, WORLD_WIDTH);
        self.pos.y = modulus(self.pos.y, WORLD_HEIGHT);
        self.speed *= 0.9;
        self.life += (self.photosynthesis - self.metabolism())*0.4;
        if rng.rand_float() < 0.2*self.motion {
            let speed = 7. * self.motion / self.weight();
            if self.intelligence > 0. {
                if let Some(feed_dir) = feed_dir {
                    self.motion(feed_dir, speed);
                } else {
                    self.random_move(rng, speed)
                }
            } else {
                self.random_move(rng, speed)
            }
        }
        self.age += 1;
        offspring
    }
    pub fn interact(biots: &mut Vec<Self>, i:usize, j:usize) {
        let dist = (biots[i].pos - biots[j].pos).length();
        if dist < 10.* (biots[i].weight() + biots[j].weight()) {
            if biots[i].stronger(&biots[j]) {
                biots[i].life += biots[j].life * 0.8;
                biots[j].life = 0.;
            }
            else if biots[j].stronger(&biots[i]) {
                biots[j].life += biots[i].life * 0.8;
                biots[i].life = 0.;
            }
        }
    }
    pub fn dead(&self) -> bool {
        self.life <= 0. || self.age >= 10000
    }
    pub fn stronger(&self, other: &Self) -> bool {
        self.attack > other.attack + other.defense * 0.8
    }
    fn set_from_genome(&mut self) {
        self.attack= self.genome.iter().filter(|&&c|c=='a').count() as f32 * 0.1;
        self.defense= self.genome.iter().filter(|&&c|c=='d').count() as f32 * 0.1;
        self.photosynthesis= self.genome.iter().filter(|&&c|c=='p').count() as f32 * 0.1;
        self.motion= self.genome.iter().filter(|&&c|c=='m').count() as f32 * 0.1;
        self.intelligence= self.genome.iter().filter(|&&c|c=='i').count() as f32 * 10.;
    }
    fn random_move(&mut self, rng: &mut Rand32, speed: f32) {
        self.motion(vec2(rng.rand_float()-0.5, rng.rand_float()-0.5).normalize(), speed);
    }
    fn motion(&mut self, dir:Vec2, speed: f32) {
        self.speed += dir *speed;
    }
    fn mutate(&mut self, rng: &mut Rand32) {
        self.genome[rng.rand_range(0..(self.genome.len() as u32)) as usize] = LETTERS[rng.rand_range(0..(LETTERS.len() as u32)) as usize];
        self.set_from_genome();
    }
    fn base_life(&self) -> f32 {
        8. * self.weight()
    }
    fn metabolism(&self) -> f32 {
        0.2*(4.5*self.attack + 2.3*self.defense + 2.5*self.motion + 0.1*self.intelligence)
    }
    fn weight(&self) -> f32 {
        self.attack + self.defense + self.photosynthesis + self.motion
    }
}

pub struct TreePoint {
    pub x:f64,
    pub y:f64,
    pub idx: usize
}

impl RTreeObject for TreePoint {
    type Envelope = AABB<[f64; 2]>;
    fn envelope(&self) -> Self::Envelope
    {
        AABB::from_point([self.x, self.y])
    }
}

impl PointDistance for TreePoint
{
fn distance_2(&self, point: &<<Self as rstar::RTreeObject>::Envelope as rstar::Envelope>::Point) -> <<<Self as rstar::RTreeObject>::Envelope as rstar::Envelope>::Point as rstar::Point>::Scalar
{
    (self.x-point[0])*(self.x-point[0]) + (self.y-point[1])*(self.y-point[1])
}
}
