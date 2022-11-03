use std::ops::Add;

use rand::Rng;
use crate::{point::Pt, Ent};



#[derive(Debug)]
pub struct Food {
    seed: f32,
    pub loc: Pt,
    pub dir: f32,
    obj: Option<Pt>,
    hunger: f32,
    age: i32,
}

impl Food {
    pub fn new(s: Option<f32>, dir: Option<f32>) -> Self {
        let seed = if let Some(sd) = s {
            sd
        } else {
            rand::thread_rng().gen()
        };
        let seceed: f32 = rand::thread_rng().gen();
        let threed: f32 = rand::thread_rng().gen();
        Self {
            seed,
            loc: Pt ((seed* -50.0)+25.0, 0.0, (seceed* -50.0)+25.0 ),
            dir: if let Some(d) = dir { d } else { seed * 360.0 },
            obj: None,
            hunger: 0.2,
            age: (threed * 3000.0 + (seceed * 3000.0)) as i32,
        }
    }
}
impl Ent for Food {
    fn tick(&mut self) {
        self.age += 1;
        self.loc = self.loc + Pt(0.0, (self.age as f32 / 60.0).sin() * 0.005, 0.0);
    }
}