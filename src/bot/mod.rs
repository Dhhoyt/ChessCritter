mod masks;
mod moves;
mod pseudomoves;
mod utils;

type BitBoard = u64;

struct BoardMove {
    from: usize,
    to: usize,
    piece: Piece,
    taken: Piece,
    white: bool,
}

enum Piece {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

struct Board {
    white_rooks: BitBoard,
    black_rooks: BitBoard,
    white_knights: BitBoard,
    black_knights: BitBoard,
    white_bishops: BitBoard,
    black_bishops: BitBoard,
    white_queens: BitBoard,
    black_queens: BitBoard,
    white_kings: BitBoard,
    black_kings: BitBoard,
    white_pawns: BitBoard,
    black_pawns: BitBoard,

    white_occupied: BitBoard,
    black_occupied: BitBoard,
    occupied: BitBoard,

    castle: BitBoard,
    en_passant: BitBoard,

    white_to_play: bool,
}

impl Board {
    fn new() -> Self {
        Board {
            black_pawns: 0,
            black_knights: 0,
            black_bishops: 0,
            black_rooks: 0,
            black_queens: 0,
            black_kings: 0,
            white_pawns: 0,
            white_knights: 0,
            white_bishops: 0,
            white_rooks: 0,
            white_queens: 0,
            white_kings: 0,

            white_occupied: 0,
            black_occupied: 0,
            occupied: 0,

            castle: 0,
            en_passant: 0,

            white_to_play: true,
        }
    }

    #[inline]
    const fn white_or_empty(&self) -> BitBoard {
        !self.black_occupied
    }

    #[inline]
    const fn black_or_empty(&self) -> BitBoard {
        !self.white_occupied
    }

    #[inline]
    pub fn hueristic(&self) -> f32 {
        let mut total: f32 = 0.;
        total +=
            (self.white_queens.count_ones() as f32 - self.black_queens.count_ones() as f32) * 9.;
        total += (self.white_rooks.count_ones() as f32 - self.black_rooks.count_ones() as f32) * 5.;
        total +=
            (self.white_bishops.count_ones() as f32 - self.black_bishops.count_ones() as f32) * 3.;
        total +=
            (self.white_knights.count_ones() as f32 - self.black_knights.count_ones() as f32) * 3.;
        total += (self.white_pawns.count_ones() as f32 - self.black_pawns.count_ones() as f32) * 1.;

        total += self.under_attack_by_white().count_ones() as f32 * 0.25;
        total -= self.under_attack_by_black().count_ones() as f32 * 0.25;
        total
    }

    pub fn what_black_piece(&self, square: usize) -> (Piece, usize) {
        let mask = (1 as BitBoard) << square;
        if mask & self.black_pawns != 0 {
            (Piece::Pawn, 100)
        } else if mask & self.black_knights != 0 {
            (Piece::Knight, 200)
        } else if mask & self.black_bishops != 0 {
            (Piece::Bishop, 300)
        } else if mask & self.black_rooks != 0 {
            (Piece::Rook, 400)
        } else if mask & self.black_queens != 0 {
            (Piece::Queen, 500)
        } else {
            panic!()
        }
    }
}
impl Default for Board {
    fn default() -> Self {
        Board {
            white_rooks: 0x0000000000000081,
            black_rooks: 0x8100000000000000,
            white_knights: 0x0000000000000042,
            black_knights: 0x4200000000000000,
            white_bishops: 0x0000000000000024,
            black_bishops: 0x2400000000000000,
            white_queens: 0x000000000000008,
            black_queens: 0x800000000000000,
            white_kings: 0x0000000000000010,
            black_kings: 0x1000000000000000,
            white_pawns: 0x000000000000ff00,
            black_pawns: 0x00ff000000000000,

            white_occupied: 0x000000000000ffff,
            black_occupied: 0xffff000000000000,
            occupied: 0xffff00000000ffff,

            castle: 0x9100000000000091,
            en_passant: 0,

            white_to_play: true,
        }
    }
}
