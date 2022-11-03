use std::ops::Add;

use crate::{point::Pt, Ent};
use cgmath::{Basis2, Rotation};
use rand::Rng;

const MOVE_STEP: f32 = 0.02;
const ROTATE_STEP: f32 = 0.2;
const HUNGER_STEP: f32 = 0.0001;

enum State {
    Germinate,
    Wander,
    Advance,
}

#[derive(Debug)]
pub struct Mop {
    seed: f32,
    pub loc: Pt,
    pub dir: f32,
    obj: Option<Pt>,
    hunger: f32,
    age: i32,
}

impl Mop {
    pub fn new(s: Option<f32>, dir: Option<f32>) -> Self {
        let seed = if let Some(sd) = s {
            sd
        } else {
            rand::thread_rng().gen()
        };
        println!("{}", seed);
        Self {
            seed,
            loc: Pt(0.0, 0.0, 0.0),
            dir: if let Some(d) = dir { d } else { seed * 360.0 },
            obj: None,
            hunger: 0.2,
            age: (seed * -1000.0) as i32,
        }
    }

    pub fn set_obj(mut self, o: Pt) -> Self {
        self.obj = Some(o);
        self
    }

    pub fn obj(&self) -> Option<Pt> {
        self.obj
    }
}
impl Ent for Mop {
    fn tick(&mut self) {
        self.age += 1;
        self.hunger += HUNGER_STEP;
        let rot: Basis2<f32> = cgmath::Rotation2::from_angle(cgmath::Deg(self.dir));
        self.dir += ROTATE_STEP;
        let state = match self.age {
            0..=14 => State::Wander,
            15.. if self.obj.is_some() => State::Advance,
            15.. => State::Wander,
            _ => State::Germinate,
        };

        match state {
            State::Germinate => (),
            State::Wander => {
                self.loc = self.loc
                    + Pt::from_vec2_h(
                        rot.rotate_vector(cgmath::Vector2::unit_x() * MOVE_STEP),
                        (self.age as f32 / 60.0).sin() * 0.005,
                    )
            }
            State::Advance => {
                let obj = self.obj.unwrap();
                let delta = (obj - self.loc).norm() * MOVE_STEP;
                self.loc = self.loc + delta;
                if (self.loc - obj).mag() < 0.01 {
                    self.obj = None;
                }
            }
        };
    }
}
