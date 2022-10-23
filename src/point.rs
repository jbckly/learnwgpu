use std::ops::Sub;

#[derive()]
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Pt(pub f32, pub f32, pub f32);

impl Pt {
    pub fn mag(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
}

impl Eq for Pt {}

impl Sub for Pt {
    type Output = Pt;
    fn sub(self, other: Pt) -> Self {
        Pt ( self.0 - other.0, self.1 - other.1, self.2 - other.2, )
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
    const P_C: Pt = Pt(-3.8, -0.2, 0.0);

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
        assert_eq!((p_b-p_a), Pt (1.0, 1.0, 1.0));
    }

    #[test]
    fn dist() {
        let p_a = P_A;
        let p_a2 = P_A;
        assert_eq!((p_a-p_a2).mag(), 0.0);
    }
}