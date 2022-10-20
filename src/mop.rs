use std::ops::Add;

use cgmath::{Basis2, Rotation};
use rand::Rng;

const MOVE_STEP: f32 = 0.02;
const ROTATE_STEP: f32 = 0.2;
const HUNGER_STEP: f32 = 0.0001;

#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}

impl Coord {
    fn from_vec2(v2: cgmath::Vector2<f32>) -> Self {
        Self {
            x: v2.x,
            y: v2.y,
        }
    }
}
impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: &self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
pub struct Mop {
    seed: f32,
    pub loc: Coord,
    pub height: f32,
    pub dir: f32,
    obj: Option<Coord>,
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
            loc: Coord { x: 0.0, y: 0.0 },
            height: 0.0,
            dir: if let Some(d) = dir {
                d
            } else {
                seed * 360.0
            },
            obj: None,
            hunger: 0.2,
            age: (seed * -1000.0) as i32,
        }
    }

    pub fn tick(&mut self) {
        self.age += 1;
        self.hunger += HUNGER_STEP;
        let rot: Basis2<f32> = cgmath::Rotation2::from_angle(cgmath::Deg(self.dir));
        self.dir += ROTATE_STEP;
        if self.age > 0 {
            self.loc = self.loc + Coord::from_vec2(rot.rotate_vector(cgmath::Vector2::unit_x() * MOVE_STEP));
            self.height = (self.age as f32 / 10.0).sin() * 0.1; // + (self.age as f32/100.0).sin() * 12.0 + (self.age as f32).sin();
        };
    }
}
