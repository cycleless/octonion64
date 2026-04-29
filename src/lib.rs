#![no_std]
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign,
    Neg, Not, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Octave(pub [u64; 8]);

impl Octave {
    pub const ZERO: Self = Self([0; 8]);

    #[inline]
    #[must_use]
    pub const fn splat(v: u64) -> Self {
        Self([v, v, v, v, v, v, v, v])
    }

    #[inline]
    #[must_use]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self([
            self.0[0].wrapping_add(rhs.0[0]),
            self.0[1].wrapping_add(rhs.0[1]),
            self.0[2].wrapping_add(rhs.0[2]),
            self.0[3].wrapping_add(rhs.0[3]),
            self.0[4].wrapping_add(rhs.0[4]),
            self.0[5].wrapping_add(rhs.0[5]),
            self.0[6].wrapping_add(rhs.0[6]),
            self.0[7].wrapping_add(rhs.0[7]),
        ])
    }

    const fn q_mul(x: [u64; 4], y: [u64; 4]) -> [u64; 4] {
        [
            x[0].wrapping_mul(y[0])
                .wrapping_sub(x[1].wrapping_mul(y[1]))
                .wrapping_sub(x[2].wrapping_mul(y[2]))
                .wrapping_sub(x[3].wrapping_mul(y[3])),
            x[0].wrapping_mul(y[1])
                .wrapping_add(x[1].wrapping_mul(y[0]))
                .wrapping_add(x[2].wrapping_mul(y[3]))
                .wrapping_sub(x[3].wrapping_mul(y[2])),
            x[0].wrapping_mul(y[2])
                .wrapping_sub(x[1].wrapping_mul(y[3]))
                .wrapping_add(x[2].wrapping_mul(y[0]))
                .wrapping_add(x[3].wrapping_mul(y[1])),
            x[0].wrapping_mul(y[3])
                .wrapping_add(x[1].wrapping_mul(y[2]))
                .wrapping_sub(x[2].wrapping_mul(y[1]))
                .wrapping_add(x[3].wrapping_mul(y[0])),
        ]
    }

    const fn q_conj(x: [u64; 4]) -> [u64; 4] {
        [
            x[0],
            0u64.wrapping_sub(x[1]),
            0u64.wrapping_sub(x[2]),
            0u64.wrapping_sub(x[3]),
        ]
    }

    #[inline]
    #[must_use]
    pub const fn mul(self, rhs: Self) -> Self {
        let a = [self.0[0], self.0[1], self.0[2], self.0[3]];
        let b = [self.0[4], self.0[5], self.0[6], self.0[7]];
        let c = [rhs.0[0], rhs.0[1], rhs.0[2], rhs.0[3]];
        let d = [rhs.0[4], rhs.0[5], rhs.0[6], rhs.0[7]];

        let ac = Self::q_mul(a, c);

        let d_conj_b = Self::q_mul(Self::q_conj(d), b);
        let da = Self::q_mul(d, a);
        let bc_conj = Self::q_mul(b, Self::q_conj(c));

        Self([
            ac[0].wrapping_sub(d_conj_b[0]),
            ac[1].wrapping_sub(d_conj_b[1]),
            ac[2].wrapping_sub(d_conj_b[2]),
            ac[3].wrapping_sub(d_conj_b[3]),
            da[0].wrapping_add(bc_conj[0]),
            da[1].wrapping_add(bc_conj[1]),
            da[2].wrapping_add(bc_conj[2]),
            da[3].wrapping_add(bc_conj[3]),
        ])
    }

    #[inline]
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        v0: u64,
        v1: u64,
        v2: u64,
        v3: u64,
        v4: u64,
        v5: u64,
        v6: u64,
        v7: u64,
    ) -> Self {
        Self([v0, v1, v2, v3, v4, v5, v6, v7])
    }

    #[inline]
    #[must_use]
    pub const fn norm_sq(&self) -> u64 {
        self.0[0]
            .wrapping_mul(self.0[0])
            .wrapping_add(self.0[1].wrapping_mul(self.0[1]))
            .wrapping_add(self.0[2].wrapping_mul(self.0[2]))
            .wrapping_add(self.0[3].wrapping_mul(self.0[3]))
            .wrapping_add(self.0[4].wrapping_mul(self.0[4]))
            .wrapping_add(self.0[5].wrapping_mul(self.0[5]))
            .wrapping_add(self.0[6].wrapping_mul(self.0[6]))
            .wrapping_add(self.0[7].wrapping_mul(self.0[7]))
    }

    #[inline]
    #[must_use]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self([
            self.0[0].wrapping_sub(rhs.0[0]),
            self.0[1].wrapping_sub(rhs.0[1]),
            self.0[2].wrapping_sub(rhs.0[2]),
            self.0[3].wrapping_sub(rhs.0[3]),
            self.0[4].wrapping_sub(rhs.0[4]),
            self.0[5].wrapping_sub(rhs.0[5]),
            self.0[6].wrapping_sub(rhs.0[6]),
            self.0[7].wrapping_sub(rhs.0[7]),
        ])
    }

    #[inline]
    #[must_use]
    pub const fn rotate_left(self, n: u32) -> Self {
        Self([
            self.0[0].rotate_left(n),
            self.0[1].rotate_left(n),
            self.0[2].rotate_left(n),
            self.0[3].rotate_left(n),
            self.0[4].rotate_left(n),
            self.0[5].rotate_left(n),
            self.0[6].rotate_left(n),
            self.0[7].rotate_left(n),
        ])
    }

    #[inline]
    #[must_use]
    pub const fn rotate_right(self, n: u32) -> Self {
        Self([
            self.0[0].rotate_right(n),
            self.0[1].rotate_right(n),
            self.0[2].rotate_right(n),
            self.0[3].rotate_right(n),
            self.0[4].rotate_right(n),
            self.0[5].rotate_right(n),
            self.0[6].rotate_right(n),
            self.0[7].rotate_right(n),
        ])
    }

    #[inline]
    #[must_use]
    pub const fn conj(self) -> Self {
        Self([
            self.0[0],
            0u64.wrapping_sub(self.0[1]),
            0u64.wrapping_sub(self.0[2]),
            0u64.wrapping_sub(self.0[3]),
            0u64.wrapping_sub(self.0[4]),
            0u64.wrapping_sub(self.0[5]),
            0u64.wrapping_sub(self.0[6]),
            0u64.wrapping_sub(self.0[7]),
        ])
    }

    #[inline]
    #[must_use]
    pub const fn scalar_mul(self, s: u64) -> Self {
        Self([
            self.0[0].wrapping_mul(s),
            self.0[1].wrapping_mul(s),
            self.0[2].wrapping_mul(s),
            self.0[3].wrapping_mul(s),
            self.0[4].wrapping_mul(s),
            self.0[5].wrapping_mul(s),
            self.0[6].wrapping_mul(s),
            self.0[7].wrapping_mul(s),
        ])
    }

    #[inline]
    #[must_use]
    pub const fn commutator(self, rhs: Self) -> Self {
        self.mul(rhs).wrapping_sub(rhs.mul(self))
    }

    #[inline]
    #[must_use]
    pub const fn associator(self, y: Self, z: Self) -> Self {
        self.mul(y).mul(z).wrapping_sub(self.mul(y.mul(z)))
    }

    #[inline]
    #[must_use]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self([
            self.0[0].wrapping_mul(rhs.0[0]),
            self.0[1].wrapping_mul(rhs.0[1]),
            self.0[2].wrapping_mul(rhs.0[2]),
            self.0[3].wrapping_mul(rhs.0[3]),
            self.0[4].wrapping_mul(rhs.0[4]),
            self.0[5].wrapping_mul(rhs.0[5]),
            self.0[6].wrapping_mul(rhs.0[6]),
            self.0[7].wrapping_mul(rhs.0[7]),
        ])
    }

    #[inline]
    #[must_use]
    pub const fn wrapping_neg(self) -> Self {
        Self([
            self.0[0].wrapping_neg(),
            self.0[1].wrapping_neg(),
            self.0[2].wrapping_neg(),
            self.0[3].wrapping_neg(),
            self.0[4].wrapping_neg(),
            self.0[5].wrapping_neg(),
            self.0[6].wrapping_neg(),
            self.0[7].wrapping_neg(),
        ])
    }

    #[inline]
    #[must_use]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self([
            self.0[0].saturating_add(rhs.0[0]),
            self.0[1].saturating_add(rhs.0[1]),
            self.0[2].saturating_add(rhs.0[2]),
            self.0[3].saturating_add(rhs.0[3]),
            self.0[4].saturating_add(rhs.0[4]),
            self.0[5].saturating_add(rhs.0[5]),
            self.0[6].saturating_add(rhs.0[6]),
            self.0[7].saturating_add(rhs.0[7]),
        ])
    }

    #[inline]
    #[must_use]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self([
            self.0[0].saturating_sub(rhs.0[0]),
            self.0[1].saturating_sub(rhs.0[1]),
            self.0[2].saturating_sub(rhs.0[2]),
            self.0[3].saturating_sub(rhs.0[3]),
            self.0[4].saturating_sub(rhs.0[4]),
            self.0[5].saturating_sub(rhs.0[5]),
            self.0[6].saturating_sub(rhs.0[6]),
            self.0[7].saturating_sub(rhs.0[7]),
        ])
    }

    #[inline]
    #[must_use]
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self([
            self.0[0].saturating_mul(rhs.0[0]),
            self.0[1].saturating_mul(rhs.0[1]),
            self.0[2].saturating_mul(rhs.0[2]),
            self.0[3].saturating_mul(rhs.0[3]),
            self.0[4].saturating_mul(rhs.0[4]),
            self.0[5].saturating_mul(rhs.0[5]),
            self.0[6].saturating_mul(rhs.0[6]),
            self.0[7].saturating_mul(rhs.0[7]),
        ])
    }

    #[inline]
    #[must_use]
    pub const fn min(self, rhs: Self) -> Self {
        Self([
            if self.0[0] < rhs.0[0] {
                self.0[0]
            } else {
                rhs.0[0]
            },
            if self.0[1] < rhs.0[1] {
                self.0[1]
            } else {
                rhs.0[1]
            },
            if self.0[2] < rhs.0[2] {
                self.0[2]
            } else {
                rhs.0[2]
            },
            if self.0[3] < rhs.0[3] {
                self.0[3]
            } else {
                rhs.0[3]
            },
            if self.0[4] < rhs.0[4] {
                self.0[4]
            } else {
                rhs.0[4]
            },
            if self.0[5] < rhs.0[5] {
                self.0[5]
            } else {
                rhs.0[5]
            },
            if self.0[6] < rhs.0[6] {
                self.0[6]
            } else {
                rhs.0[6]
            },
            if self.0[7] < rhs.0[7] {
                self.0[7]
            } else {
                rhs.0[7]
            },
        ])
    }

    #[inline]
    #[must_use]
    pub const fn max(self, rhs: Self) -> Self {
        Self([
            if self.0[0] > rhs.0[0] {
                self.0[0]
            } else {
                rhs.0[0]
            },
            if self.0[1] > rhs.0[1] {
                self.0[1]
            } else {
                rhs.0[1]
            },
            if self.0[2] > rhs.0[2] {
                self.0[2]
            } else {
                rhs.0[2]
            },
            if self.0[3] > rhs.0[3] {
                self.0[3]
            } else {
                rhs.0[3]
            },
            if self.0[4] > rhs.0[4] {
                self.0[4]
            } else {
                rhs.0[4]
            },
            if self.0[5] > rhs.0[5] {
                self.0[5]
            } else {
                rhs.0[5]
            },
            if self.0[6] > rhs.0[6] {
                self.0[6]
            } else {
                rhs.0[6]
            },
            if self.0[7] > rhs.0[7] {
                self.0[7]
            } else {
                rhs.0[7]
            },
        ])
    }

    #[inline]
    #[must_use]
    pub const fn swap_bytes(self) -> Self {
        Self([
            self.0[0].swap_bytes(),
            self.0[1].swap_bytes(),
            self.0[2].swap_bytes(),
            self.0[3].swap_bytes(),
            self.0[4].swap_bytes(),
            self.0[5].swap_bytes(),
            self.0[6].swap_bytes(),
            self.0[7].swap_bytes(),
        ])
    }

    #[inline]
    #[must_use]
    pub const fn reverse_bits(self) -> Self {
        Self([
            self.0[0].reverse_bits(),
            self.0[1].reverse_bits(),
            self.0[2].reverse_bits(),
            self.0[3].reverse_bits(),
            self.0[4].reverse_bits(),
            self.0[5].reverse_bits(),
            self.0[6].reverse_bits(),
            self.0[7].reverse_bits(),
        ])
    }

    #[inline]
    #[must_use]
    pub const fn count_ones(&self) -> u32 {
        self.0[0].count_ones()
            + self.0[1].count_ones()
            + self.0[2].count_ones()
            + self.0[3].count_ones()
            + self.0[4].count_ones()
            + self.0[5].count_ones()
            + self.0[6].count_ones()
            + self.0[7].count_ones()
    }
}

impl BitXor for Octave {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self([
            self.0[0] ^ rhs.0[0],
            self.0[1] ^ rhs.0[1],
            self.0[2] ^ rhs.0[2],
            self.0[3] ^ rhs.0[3],
            self.0[4] ^ rhs.0[4],
            self.0[5] ^ rhs.0[5],
            self.0[6] ^ rhs.0[6],
            self.0[7] ^ rhs.0[7],
        ])
    }
}

impl BitXorAssign for Octave {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0[0] ^= rhs.0[0];
        self.0[1] ^= rhs.0[1];
        self.0[2] ^= rhs.0[2];
        self.0[3] ^= rhs.0[3];
        self.0[4] ^= rhs.0[4];
        self.0[5] ^= rhs.0[5];
        self.0[6] ^= rhs.0[6];
        self.0[7] ^= rhs.0[7];
    }
}

impl BitOr for Octave {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self([
            self.0[0] | rhs.0[0],
            self.0[1] | rhs.0[1],
            self.0[2] | rhs.0[2],
            self.0[3] | rhs.0[3],
            self.0[4] | rhs.0[4],
            self.0[5] | rhs.0[5],
            self.0[6] | rhs.0[6],
            self.0[7] | rhs.0[7],
        ])
    }
}

impl BitAnd for Octave {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self([
            self.0[0] & rhs.0[0],
            self.0[1] & rhs.0[1],
            self.0[2] & rhs.0[2],
            self.0[3] & rhs.0[3],
            self.0[4] & rhs.0[4],
            self.0[5] & rhs.0[5],
            self.0[6] & rhs.0[6],
            self.0[7] & rhs.0[7],
        ])
    }
}

impl Not for Octave {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self([
            !self.0[0], !self.0[1], !self.0[2], !self.0[3], !self.0[4], !self.0[5], !self.0[6],
            !self.0[7],
        ])
    }
}

impl Shl<u32> for Octave {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u32) -> Self {
        Self([
            self.0[0] << rhs,
            self.0[1] << rhs,
            self.0[2] << rhs,
            self.0[3] << rhs,
            self.0[4] << rhs,
            self.0[5] << rhs,
            self.0[6] << rhs,
            self.0[7] << rhs,
        ])
    }
}

impl Shr<u32> for Octave {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u32) -> Self {
        Self([
            self.0[0] >> rhs,
            self.0[1] >> rhs,
            self.0[2] >> rhs,
            self.0[3] >> rhs,
            self.0[4] >> rhs,
            self.0[5] >> rhs,
            self.0[6] >> rhs,
            self.0[7] >> rhs,
        ])
    }
}

impl Add for Octave {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }
}

impl Sub for Octave {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }
}

impl Mul for Octave {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.mul(rhs)
    }
}

impl Neg for Octave {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        self.wrapping_neg()
    }
}

impl AddAssign for Octave {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.wrapping_add(rhs);
    }
}

impl SubAssign for Octave {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.wrapping_sub(rhs);
    }
}

impl MulAssign for Octave {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl BitAndAssign for Octave {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0[0] &= rhs.0[0];
        self.0[1] &= rhs.0[1];
        self.0[2] &= rhs.0[2];
        self.0[3] &= rhs.0[3];
        self.0[4] &= rhs.0[4];
        self.0[5] &= rhs.0[5];
        self.0[6] &= rhs.0[6];
        self.0[7] &= rhs.0[7];
    }
}

impl BitOrAssign for Octave {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0[0] |= rhs.0[0];
        self.0[1] |= rhs.0[1];
        self.0[2] |= rhs.0[2];
        self.0[3] |= rhs.0[3];
        self.0[4] |= rhs.0[4];
        self.0[5] |= rhs.0[5];
        self.0[6] |= rhs.0[6];
        self.0[7] |= rhs.0[7];
    }
}

impl ShlAssign<u32> for Octave {
    #[inline]
    fn shl_assign(&mut self, rhs: u32) {
        self.0[0] <<= rhs;
        self.0[1] <<= rhs;
        self.0[2] <<= rhs;
        self.0[3] <<= rhs;
        self.0[4] <<= rhs;
        self.0[5] <<= rhs;
        self.0[6] <<= rhs;
        self.0[7] <<= rhs;
    }
}

impl ShrAssign<u32> for Octave {
    #[inline]
    fn shr_assign(&mut self, rhs: u32) {
        self.0[0] >>= rhs;
        self.0[1] >>= rhs;
        self.0[2] >>= rhs;
        self.0[3] >>= rhs;
        self.0[4] >>= rhs;
        self.0[5] >>= rhs;
        self.0[6] >>= rhs;
        self.0[7] >>= rhs;
    }
}
