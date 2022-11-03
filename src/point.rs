use std::ops::{Sub, Add, Mul, Div};

#[derive()]
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Pt(pub f32, pub f32, pub f32);

impl Pt {
    pub fn mag(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
    pub fn norm(&self) -> Pt {
        *self / self.mag()
    }
    pub fn from_vec2(v2: cgmath::Vector2<f32>) -> Self {
        Pt::from_vec2_h(v2, 0.0)
    }

    pub fn from_vec2_h(v2: cgmath::Vector2<f32>, h: f32) -> Self {
        Pt(v2.x, h, v2.y)
    }
}

impl Eq for Pt {}

impl Add for Pt {
    type Output = Pt;
    fn add(self, other: Pt) -> Self {
        Pt ( self.0 + other.0, self.1 + other.1, self.2 + other.2, )
    }
}

impl Sub for Pt {
    type Output = Pt;
    fn sub(self, other: Pt) -> Self {
        Pt ( self.0 - other.0, self.1 - other.1, self.2 - other.2, )
    }
}

impl Mul<f32> for Pt {
    type Output = Pt;
    fn mul(self, other: f32) -> Self {
        Pt ( self.0 * other, self.1 * other, self.2 * other )
    }
}

impl Div<f32> for Pt {
    type Output = Pt;
    fn div(self, other: f32) -> Self {
        Pt ( self.0 / other, self.1 / other, self.2 / other )
    }
}

// impl Sub for Pt {
//     type Output = Pt;
//     fn sub(self, other: Pt) -> Self {
//         Pt ( self.0 - other.0, self.1 - other.1, self.2 - other.2, )
//     }
// }

#[cfg(test)]
mod tests {
    use super::Pt;
    const P_A: Pt = Pt(1.0, 1.0, 0.0);
    const P_B: Pt = Pt(2.0, 2.0, 0.0);
    const P_C: Pt = Pt(-3.8, -0.2, 3.9);

    #[test]
    fn d_f_o() {
        let p_a = P_A;
        assert_eq!(p_a.mag(), 1.4142135);
    }

    #[test]
    fn sub() {
        let p_a = P_A;
        let p_a2 = P_A;
        assert_eq!((p_a-p_a2), Pt (0.0, 0.0, 0.0));
    }
    #[test]
    fn sub_2() {
        let p_a = P_A;
        let p_b = P_B;
        assert_eq!((p_b-p_a), Pt (1.0, 1.0, 0.0));
    }

    #[test]
    fn add() {
        let p_a = P_A;
        let p_b = P_B;
        assert_eq!((p_b+p_a), Pt (3.0, 3.0, 0.0));
    }

    #[test]
    fn mul() {
        let p_c = P_C;
        assert_eq!((p_c*2.0), Pt(-7.6, -0.4, 7.8));
    }

    #[test]
    fn div() {
        let p_c = P_C;
        assert_eq!((p_c/2.0), Pt(-1.9, -0.1, 1.95));
    }

    #[test]
    fn div_math() {
        let p_c = P_C;
        assert_eq!(p_c/(p_c.mag()*2.0), p_c.norm()/2.0);
    }

    #[test]
    fn dist() {
        let p_a = P_A;
        let p_a2 = P_A;
        assert_eq!((p_a-p_a2).mag(), 0.0);
    }
}

#[cfg(test)]
mod perftest1 {
    use super::Pt;
    const P_C: Pt = Pt(-3.8, -0.2, 3.9);

    #[test]
    fn perftest1() {
        let p_c = P_C;
        for _ in 0../*100_000_00*/0 {
            // let _ = p_c/(p_c.mag()*2.0);
            let _ = p_c.norm()/2.0;
        }
    }
}
