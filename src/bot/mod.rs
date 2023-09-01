use self::utils::*;

mod black_moves;
mod masks;
mod pseudomoves;
mod utils;
mod white_moves;

type BitBoard = u64;

#[derive(Debug)]
pub struct BoardMove {
    from: usize,
    to: usize,
    piece: Piece,
    taken: Piece,
    promotion: Piece,
    white: bool,
    en_passant: bool,
    last_castle: BitBoard,
    last_en_passant: BitBoard,
    last_white_occupied: BitBoard,
    last_black_occupied: BitBoard,
    last_occupied: BitBoard,
}

#[derive(Clone, Debug)]
pub enum Piece {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Board {
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

    pub fn what_white_piece(&self, square: usize) -> (Piece, usize) {
        let mask = (1 as BitBoard) << square;
        if mask & self.white_pawns != 0 {
            (Piece::Pawn, 100)
        } else if mask & self.white_knights != 0 {
            (Piece::Knight, 200)
        } else if mask & self.white_bishops != 0 {
            (Piece::Bishop, 300)
        } else if mask & self.white_rooks != 0 {
            (Piece::Rook, 400)
        } else if mask & self.white_queens != 0 {
            (Piece::Queen, 500)
        } else {
            panic!()
        }
    }

    pub fn make_move(&mut self, board_move: &BoardMove) {
        let to: BitBoard = 1 << board_move.to;
        let from: BitBoard = 1 << board_move.from;
        let mask: BitBoard = from | to;
        self.en_passant = 0;
        if board_move.white {
            match board_move.piece {
                Piece::None => panic!(),
                Piece::Pawn => {
                    self.white_pawns ^= mask;
                    if south_one(south_one(to)) == from {
                        self.en_passant = south_one(to);
                    }
                }
                Piece::Knight => self.white_knights ^= mask,
                Piece::Bishop => self.white_bishops ^= mask,
                Piece::Rook => {
                    self.white_rooks ^= mask;
                    self.castle &= !from
                }
                Piece::Queen => self.white_queens ^= mask,
                Piece::King => {
                    self.white_kings ^= mask;
                    self.castle &= !from;
                }
            }
            self.white_occupied ^= from;
            self.white_occupied |= to;
            self.occupied ^= from;
            self.occupied |= to;
            match board_move.taken {
                Piece::None => (),
                Piece::Pawn => {
                    if board_move.en_passant {
                        self.black_pawns ^= south_one(to);
                        self.black_occupied ^= south_one(to);
                    } else {
                        self.black_pawns ^= to;
                        self.black_occupied ^= to;
                    }
                }
                Piece::Knight => {
                    self.black_knights ^= to;
                    self.black_occupied ^= to;
                }
                Piece::Bishop => {
                    self.black_bishops ^= to;
                    self.black_occupied ^= to;
                }
                Piece::Rook => {
                    self.black_rooks ^= to;
                    self.black_occupied ^= to;
                }
                Piece::Queen => {
                    self.black_queens ^= to;
                    self.black_occupied ^= to;
                }
                Piece::King => panic!(),
            }
            self.white_to_play = false;
        } else {
            match board_move.piece {
                Piece::None => panic!(),
                Piece::Pawn => {
                    self.black_pawns ^= mask;
                    if north_one(north_one(to)) == from {
                        self.en_passant = north_one(to);
                    }
                }
                Piece::Knight => self.black_knights ^= mask,
                Piece::Bishop => self.black_bishops ^= mask,
                Piece::Rook => {
                    self.black_rooks ^= mask;
                    self.castle &= !from
                }
                Piece::Queen => self.black_queens ^= mask,
                Piece::King => {
                    self.black_kings ^= mask;
                    self.castle &= !from
                }
            }
            self.black_occupied ^= from;
            self.black_occupied |= to;
            self.occupied ^= from;
            self.occupied |= to;
            match board_move.taken {
                Piece::None => (),
                Piece::Pawn => {
                    if board_move.en_passant {
                        self.white_pawns ^= north_one(to);
                        self.white_occupied ^= north_one(to);
                    } else {
                        self.white_pawns ^= to;
                        self.white_occupied ^= to;
                    }
                }
                Piece::Knight => {
                    self.white_knights ^= to;
                    self.white_occupied ^= to;
                }
                Piece::Bishop => {
                    self.white_bishops ^= to;
                    self.white_occupied ^= to;
                }
                Piece::Rook => {
                    self.white_rooks ^= to;
                    self.white_occupied ^= to;
                }
                Piece::Queen => {
                    self.white_queens ^= to;
                    self.white_occupied ^= to;
                }
                Piece::King => panic!(),
            }
            self.white_to_play = true;
        }
    }

    pub fn undo_move(&mut self, board_move: &BoardMove) {
        let to: BitBoard = 1 << board_move.to;
        let from: BitBoard = 1 << board_move.from;
        let mask: BitBoard = from | to;
        self.en_passant = board_move.last_en_passant;
        self.castle = board_move.last_castle;
        self.occupied = board_move.last_occupied;
        self.white_occupied = board_move.last_white_occupied;
        self.black_occupied = board_move.last_black_occupied;
        if board_move.white {
            self.white_to_play = true;
            match board_move.piece {
                Piece::None => panic!(),
                Piece::Pawn => self.white_pawns ^= mask,
                Piece::Knight => self.white_knights ^= mask,
                Piece::Bishop => self.white_bishops ^= mask,
                Piece::Rook => self.white_rooks ^= mask,
                Piece::Queen => self.white_queens ^= mask,
                Piece::King => self.white_kings ^= mask,
            }
            match board_move.taken {
                Piece::None => (),
                Piece::Pawn => {
                    if board_move.en_passant {
                        self.black_pawns |= south_one(to)
                    } else {
                        self.black_pawns |= to
                    }
                }
                Piece::Knight => self.black_knights |= to,
                Piece::Bishop => self.black_bishops |= to,
                Piece::Rook => self.black_rooks |= to,
                Piece::Queen => self.black_queens |= to,
                Piece::King => self.black_kings |= to,
            }
        } else {
            self.white_to_play = false;
            match board_move.piece {
                Piece::None => panic!(),
                Piece::Pawn => self.black_pawns ^= mask,
                Piece::Knight => self.black_knights ^= mask,
                Piece::Bishop => self.black_bishops ^= mask,
                Piece::Rook => self.black_rooks ^= mask,
                Piece::Queen => self.black_queens ^= mask,
                Piece::King => self.black_kings ^= mask,
            }
            match board_move.taken {
                Piece::None => (),
                Piece::Pawn => {
                    if board_move.en_passant {
                        self.white_pawns |= north_one(to)
                    } else {
                        self.white_pawns |= to
                    }
                }
                Piece::Knight => self.white_knights |= to,
                Piece::Bishop => self.white_bishops |= to,
                Piece::Rook => self.white_rooks |= to,
                Piece::Queen => self.white_queens |= to,
                Piece::King => self.white_kings |= to,
            }
        }
    }
    pub fn perft(&mut self, depth: usize) -> usize {
        if self.white_to_play {
            if depth == 0 {
                return self.white_moves().len();
            }
            let mut total = 0;
            for i in self.white_moves() {
                self.make_move(&i.0);
                total += self.perft(depth - 1);
                self.undo_move(&i.0);
            }
            return total;
        } else {
            if depth == 0 {
                return self.black_moves().len();
            }
            let mut total = 0;
            for i in self.black_moves() {
                self.make_move(&i.0);
                total += self.perft(depth - 1);
                self.undo_move(&i.0);
            }
            return total;
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

impl BoardMove {
    pub fn readable(&self) -> String {
        let from = square_string(self.from);
        let to = square_string(self.to);
        let piece = match self.piece {
            Piece::None => "None",
            Piece::Pawn => "Pawn",
            Piece::Knight => "Knight",
            Piece::Bishop => "Bishop",
            Piece::Rook => "Rook",
            Piece::Queen => "Queen",
            Piece::King => "King",
        };
        let turn = if self.white { "White" } else { "Black" };
        format!("{} moved {} from {} to {} ", turn, piece, from, to)
    }
}

fn square_string(square: usize) -> String {
    let mut res = String::new();
    match square % 8 {
        0 => res.push('a'),
        1 => res.push('b'),
        2 => res.push('c'),
        3 => res.push('d'),
        4 => res.push('e'),
        5 => res.push('f'),
        6 => res.push('g'),
        7 => res.push('h'),
        _ => panic!(),
    };
    match square / 8 {
        0 => res.push('1'),
        1 => res.push('2'),
        2 => res.push('3'),
        3 => res.push('4'),
        4 => res.push('5'),
        5 => res.push('6'),
        6 => res.push('7'),
        7 => res.push('8'),
        _ => panic!(),
    };
    res
}