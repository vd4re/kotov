#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const fn new() -> Self {
        Self(0)
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

impl core::fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut output = String::new();
        for i in 0..64 {
            if i % 8 == 0 && i != 0 {
                output.push('\n');
            }
            output.push(if self.is_set(i) { '1' } else { '0' });
        }
        write!(f, "{} = {}", output, self.0)
    }
}
