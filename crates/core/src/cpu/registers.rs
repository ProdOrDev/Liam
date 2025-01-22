//! The status flags and register file of the processor.

/// The status flags.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags {
    raw: u8,
}

impl Flags {
    /// The carry flag.
    pub const C: Self = Self { raw: 1 << 4 };

    /// The half-carry flag.
    pub const H: Self = Self { raw: 1 << 5 };

    /// The subtraction flag.
    pub const N: Self = Self { raw: 1 << 6 };

    /// The zero flag.
    pub const Z: Self = Self { raw: 1 << 7 };

    /// An empty set of flags.
    pub const EMPTY: Self = Self { raw: 0 };

    /// Create a new set of flags from their bit representation.
    #[must_use]
    #[inline]
    pub fn from_bits(bits: u8) -> Self {
        Self { raw: bits }
    }

    /// Return the bit representation of this set of flags.
    #[must_use]
    #[inline]
    pub fn into_bits(self) -> u8 {
        self.raw & 0b1111_0000
    }
}

impl std::fmt::Debug for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.raw >> 4 {
            0b0000 => "<empty>",
            0b0001 => "C",
            0b0010 => "H",
            0b0011 => "H | C",
            0b0100 => "N",
            0b0101 => "N | C",
            0b0110 => "N | H",
            0b0111 => "N | H | C",
            0b1000 => "Z",
            0b1001 => "Z | C",
            0b1010 => "Z | H",
            0b1011 => "Z | H | C",
            0b1100 => "Z | N",
            0b1101 => "Z | N | C",
            0b1110 => "Z | N | H",
            0b1111 => "Z | N | H | C",
            _ => unreachable!(),
        })
    }
}

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

impl BitAnd for Flags {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            raw: self.raw & rhs.raw,
        }
    }
}

impl BitAndAssign for Flags {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.raw &= rhs.raw;
    }
}

impl BitOr for Flags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            raw: self.raw | rhs.raw,
        }
    }
}

impl BitOrAssign for Flags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.raw |= rhs.raw;
    }
}

impl BitXor for Flags {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            raw: self.raw ^ rhs.raw,
        }
    }
}

impl BitXorAssign for Flags {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.raw ^= rhs.raw;
    }
}

impl Not for Flags {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self { raw: !self.raw }
    }
}

/// The register file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Registers {
    /// The 8-bit flags register.
    pub f: Flags,
    /// The 8-bit accumulator.
    pub a: u8,
    /// The 8-bit C register.
    pub c: u8,
    /// The 8-bit B register.
    pub b: u8,
    /// The 8-bit E register.
    pub e: u8,
    /// The 8-bit D register.
    pub d: u8,
    /// The 8-bit L register.
    pub l: u8,
    /// The 8-bit H register.
    pub h: u8,
    /// The 16-bit program counter.
    pub pc: u16,
    /// The 16-bit stack pointer.
    pub sp: u16,
}

impl Registers {
    /// Set the value of the 16-bit AF register.
    #[inline]
    pub fn set_af(&mut self, value: u16) {
        let [f, a] = value.to_le_bytes();
        self.f = Flags::from_bits(f);
        self.a = a;
    }

    /// Return the value of the 16-bit AF register.
    #[must_use]
    #[inline]
    pub fn af(&self) -> u16 {
        u16::from_le_bytes([self.f.into_bits(), self.a])
    }

    /// Set the value of the 16-bit BC register.
    #[inline]
    pub fn set_bc(&mut self, value: u16) {
        let [c, b] = value.to_le_bytes();
        self.c = c;
        self.b = b;
    }

    /// Return the value of the 16-bit BC register.
    #[must_use]
    #[inline]
    pub fn bc(&self) -> u16 {
        u16::from_le_bytes([self.c, self.b])
    }

    /// Set the value of the 16-bit DE register.
    #[inline]
    pub fn set_de(&mut self, value: u16) {
        let [e, d] = value.to_le_bytes();
        self.e = e;
        self.d = d;
    }

    /// Return the value of the 16-bit DE register.
    #[must_use]
    #[inline]
    pub fn de(&self) -> u16 {
        u16::from_le_bytes([self.e, self.d])
    }

    /// Set the value of the 16-bit HL register.
    #[inline]
    pub fn set_hl(&mut self, value: u16) {
        let [l, h] = value.to_le_bytes();
        self.l = l;
        self.h = h;
    }

    /// Return the value of the 16-bit HL register.
    #[must_use]
    #[inline]
    pub fn hl(&self) -> u16 {
        u16::from_le_bytes([self.l, self.h])
    }
}
