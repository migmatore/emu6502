use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct StatusFlags(u8);

impl StatusFlags {
    const SIGN_BIT: u8 = 0x80;

    pub const CARRY: Self = Self(1 << 0);
    pub const ZERO: Self = Self(1 << 1);
    pub const INTERRUPTED: Self = Self(1 << 2);
    pub const DECIMAL: Self = Self(1 << 3);
    pub const BREAK: Self = Self(1 << 4);
    pub const UNUSED: Self = Self(1 << 5);
    pub const OVERFLOW: Self = Self(1 << 6);
    pub const NEGATIVE: Self = Self(1 << 7);

    pub fn bits(self) -> u8 {
        self.0
    }

    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    pub fn set(&mut self, other: Self, value: bool) {
        if value {
            *self |= other;
        } else {
            *self &= !other;
        }
    }

    pub fn update_zn(&mut self, value: u8) {
        self.set(Self::ZERO, value == 0);
        self.set(Self::NEGATIVE, (value & Self::SIGN_BIT) != 0);
    }
}

impl std::ops::BitOr for StatusFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for StatusFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl std::ops::BitAnd for StatusFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for StatusFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl std::ops::Not for StatusFlags {
    type Output = Self;

    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl fmt::Display for StatusFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let names = "NV-BDIZC";

        for (i, ch) in names.chars().enumerate() {
            let bit = 7 - i;
            let is_set = self.0 & (1 << bit) != 0;
            let c = if is_set { ch } else { ch.to_ascii_lowercase() };
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_clear_flag() {
        let mut f = StatusFlags::default();

        f.set(StatusFlags::NEGATIVE, true);
        assert!(f.contains(StatusFlags::NEGATIVE));

        f.set(StatusFlags::NEGATIVE, false);
        assert!(!f.contains(StatusFlags::NEGATIVE));
    }
}
