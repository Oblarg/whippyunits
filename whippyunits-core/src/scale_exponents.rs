/// Prime factorization into powers of 2, 3, 5, and pi.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaleExponents(pub [i16; 4]);

impl ScaleExponents {
    /// `1`.
    pub const IDENTITY: Self = Self([0; 4]);

    /// A power of 2.
    pub const fn _2(power: i16) -> Self {
        Self([power, 0, 0, 0])
    }

    /// A power of 6.
    pub const fn _6(power: i16) -> Self {
        // We factorize 6 as 2 * 3 so we split the power cross those.
        Self([power, power, 0, 0])
    }

    /// A power of 10.
    pub const fn _10(power: i16) -> Self {
        // We factorize 10 as 2 * 5 so we split the power cross those.
        Self([power, 0, power, 0])
    }

    pub const fn mul(&self, rhs: Self) -> Self {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }

    pub const fn log10(&self) -> Option<i16> {
        if let [x, 0, y, 0] = self.0
            && x == y
        {
            Some(x)
        } else {
            None
        }
    }

    pub const fn scalar_exp(&self, rhs: i16) -> Self {
        Self([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
        ])
    }

    pub const fn neg(&self) -> Self {
        Self([-self.0[0], -self.0[1], -self.0[2], -self.0[3]])
    }
}
