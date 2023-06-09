pub mod bitboard;

pub enum Piece {
    King,
    Rook,
    Bishop,
    Queen,
    Knight,
    Pawn,
}

pub enum Color {
    White,
    Black,
}

pub type Tile = (Color, Piece);

#[rustfmt::skip]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,                                     
    A4, B4, C4, D4, E4, F4, G4, H4,                                     
    A3, B3, C3, D3, E3, F3, G3, H3,                                     
    A2, B2, C2, D2, E2, F2, G2, H2,                                     
    A1, B1, C1, D1, E1, F1, G1, H1,
}

impl const From<u8> for Square {
    fn from(i: u8) -> Self {
        assert!(Square::A8 as u8 <= i && i <= Square::H1 as u8);
        unsafe { core::mem::transmute(i) }
    }
}

#[rustfmt::skip]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum File {
    A, B, C, D, E, F, G, H,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8_square() {
        assert_eq!(Square::from(0), Square::A8);
        assert_eq!(Square::from(63), Square::H1);

        assert!(std::panic::catch_unwind(|| Square::from(64)).is_err());
    }
}
