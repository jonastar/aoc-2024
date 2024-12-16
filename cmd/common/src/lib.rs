use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct IVec2 {
    pub x: i64,
    pub y: i64,
}

impl Add<IVec2> for IVec2 {
    type Output = IVec2;

    fn add(self, rhs: IVec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<IVec2> for IVec2 {
    fn add_assign(&mut self, rhs: IVec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<IVec2> for IVec2 {
    type Output = IVec2;

    fn sub(self, rhs: IVec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl SubAssign<IVec2> for IVec2 {
    fn sub_assign(&mut self, rhs: IVec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
