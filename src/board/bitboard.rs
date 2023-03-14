#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const fn new(i: u64) -> Self {
        Self(i)
    }

    #[inline(always)]
    pub const fn is_set(&self, i: u8) -> bool {
        (self.0 & (1 << i)) != 0
    }

    #[inline(always)]
    pub fn set(&mut self, i: u8) {
        self.0 |= 1 << i
    }

    #[inline(always)]
    pub fn pop(&mut self, i: u8) {
        self.0 &= !(1 << i)
    }

    #[inline(always)]
    pub const fn ls1b(&self) -> u32 {
        self.0.trailing_zeros() // TODO: Check if _tzcnt_u64 (BMI1)
    }

    #[inline(always)]
    pub const fn ms1b(&self) -> u32 {
        self.0.leading_zeros() // TODO: Check if _lzcnt_u64 (BMI1)
    }

    #[inline(always)]
    pub const fn pop_cnt(&self) -> u32 {
        self.0.count_ones()
    }
}

impl const From<u8> for Bitboard {
    fn from(i: u8) -> Self {
        Self(1 << i)
    }
}

impl const From<super::Square> for Bitboard {
    fn from(square: super::Square) -> Self {
        Self(1 << square as u64)
    }
}

impl const From<super::File> for Bitboard {
    fn from(file: super::File) -> Self {
        use const_for::*;

        let mut i = 0;

        const_for!(r in 0..8 => {
            const_for!(f in 0..8 => {
                if f == file as u8 {
                    i |= 1 << r * 8 + f;
                }
            });
        });

        Self(i)
    }
}

impl core::fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut output = String::new();
        for i in 0..64 {
            if i % 8 == 0 && i != 0 {
                output.push('\n');
            }
            output.push(if self.is_set(i) { '1' } else { '0' });
        }
        write!(f, "\n{} = {}", output, self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitboard_from_square() {
        assert_eq!(Bitboard::from(super::super::Square::A8), Bitboard::new(1));
        assert_eq!(
            Bitboard::from(super::super::Square::H1),
            Bitboard::new(1 << 63)
        );
    }

    #[test]
    fn bitboard_from_file() {
        assert_eq!(
            Bitboard::from(super::super::File::A),
            Bitboard::new(72340172838076673)
        );
        assert_eq!(
            Bitboard::from(super::super::File::H),
            Bitboard::new(9259542123273814144)
        );
    }
}
