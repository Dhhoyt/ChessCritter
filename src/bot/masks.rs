use super::pseudomoves::*;
use super::utils::*;
use super::{BitBoard, Board};

impl Board {
    #[inline]
    pub fn black_checkmask(&self) -> BitBoard {
        let king_square = self.black_kings.trailing_zeros() as usize;
        let mut checkmask: BitBoard = 0xFFFFFFFFFFFFFFFF;
        for i in BitBoardIter(self.white_queens) {
            if queen_moves(i, !self.occupied) & self.black_kings != 0 {
                checkmask &= PATH_BETWEEN[king_square][i];
            }
        }
        for i in BitBoardIter(self.white_rooks) {
            if rook_moves(i, !self.occupied) & self.black_kings != 0 {
                checkmask &= PATH_BETWEEN[king_square][i];
            }
        }
        for i in BitBoardIter(self.white_bishops) {
            if bishop_moves(i, !self.occupied) & self.black_kings != 0 {
                checkmask &= PATH_BETWEEN[king_square][i];
            }
        }
        for i in BitBoardIter(self.white_knights) {
            if KNIGHT_MOVES[i] & self.black_kings != 0 {
                checkmask &= (1 as BitBoard) << i;
            }
        }
        for i in BitBoardIter(self.white_pawns) {
            if PAWN_ATTACKS[0][i] & self.black_kings != 0 {
                checkmask &= (1 as BitBoard) << i;
            }
        }
        checkmask
    }

    #[inline]
    pub fn black_pinmask_hv(&self) -> BitBoard {
        let king_square = self.black_kings.trailing_zeros() as usize;
        let mut pinmask: BitBoard = 0;
        for i in BitBoardIter(self.white_queens) {
            let xray = rook_xray(i, !self.occupied);
            if xray & self.black_kings != 0 {
                pinmask |= PATH_BETWEEN[king_square][i];
            }
        }
        for i in BitBoardIter(self.white_rooks) {
            let xray = rook_xray(i, !self.occupied);
            if xray & self.black_kings != 0 {
                pinmask |= PATH_BETWEEN[king_square][i];
            }
        }
        pinmask
    }

    #[inline]
    pub fn black_pinmask_d(&self) -> BitBoard {
        let king_square = self.black_kings.trailing_zeros() as usize;
        let mut pinmask: BitBoard = 0;
        for i in BitBoardIter(self.white_queens) {
            let xray = bishop_xray(i, !self.occupied);
            if xray & self.black_kings != 0 {
                pinmask |= PATH_BETWEEN[king_square][i];
            }
        }
        for i in BitBoardIter(self.white_bishops) {
            let xray = bishop_xray(i, !self.occupied);
            if xray & self.black_kings != 0 {
                pinmask |= PATH_BETWEEN[king_square][i];
            }
        }
        pinmask
    }

    #[inline]
    pub fn under_attack_by_black(&self) -> BitBoard {
        let mut res = 0;
        for i in BitBoardIter(self.black_queens) {
            res |= queen_moves(i, !(self.occupied & !self.white_kings));
        }
        for i in BitBoardIter(self.black_rooks) {
            res |= rook_moves(i, !(self.occupied & !self.white_kings));
        }
        for i in BitBoardIter(self.black_bishops) {
            res |= bishop_moves(i, !(self.occupied & !self.white_kings));
        }
        for i in BitBoardIter(self.black_knights) {
            res |= KNIGHT_MOVES[i];
        }
        for i in BitBoardIter(self.black_kings) {
            res |= KING_MOVES[i];
        }
        for i in BitBoardIter(self.black_pawns) {
            res |= PAWN_ATTACKS[1][i];
        }
        res
    }

    #[inline]
    pub fn white_checkmask(&self) -> BitBoard {
        let king_square = self.white_kings.trailing_zeros() as usize;
        let mut checkmask: BitBoard = 0xFFFFFFFFFFFFFFFF;
        for i in BitBoardIter(self.black_queens) {
            if queen_moves(i, !self.occupied) & self.white_kings != 0 {
                checkmask &= PATH_BETWEEN[king_square][i];
            }
        }
        for i in BitBoardIter(self.black_rooks) {
            if rook_moves(i, !self.occupied) & self.white_kings != 0 {
                checkmask &= PATH_BETWEEN[king_square][i];
            }
        }
        for i in BitBoardIter(self.black_bishops) {
            if bishop_moves(i, !self.occupied) & self.white_kings != 0 {
                checkmask &= PATH_BETWEEN[king_square][i];
            }
        }
        for i in BitBoardIter(self.black_knights) {
            if KNIGHT_MOVES[i] & self.white_kings != 0 {
                checkmask &= (1 as BitBoard) << i;
            }
        }
        for i in BitBoardIter(self.black_pawns) {
            if PAWN_ATTACKS[1][i] & self.white_kings != 0 {
                checkmask &= (1 as BitBoard) << i;
            }
        }
        checkmask
    }

    #[inline]
    pub fn white_pinmask_hv(&self) -> BitBoard {
        let king_square = self.white_kings.trailing_zeros() as usize;
        let mut pinmask: BitBoard = 0;
        for i in BitBoardIter(self.black_queens) {
            let xray = rook_xray(i, !self.occupied);
            if xray & self.white_kings != 0 {
                pinmask |= PATH_BETWEEN[king_square][i];
            }
        }
        for i in BitBoardIter(self.black_rooks) {
            let xray = rook_xray(i, !self.occupied);
            if xray & self.white_kings != 0 {
                pinmask |= PATH_BETWEEN[king_square][i];
            }
        }
        pinmask
    }

    #[inline]
    pub fn white_pinmask_d(&self) -> BitBoard {
        let king_square = self.white_kings.trailing_zeros() as usize;
        let mut pinmask: BitBoard = 0;
        for i in BitBoardIter(self.black_queens) {
            let xray = bishop_xray(i, !self.occupied);
            if xray & self.white_kings != 0 {
                pinmask |= PATH_BETWEEN[king_square][i];
            }
        }
        for i in BitBoardIter(self.black_bishops) {
            let xray = bishop_xray(i, !self.occupied);
            if xray & self.white_kings != 0 {
                pinmask |= PATH_BETWEEN[king_square][i];
            }
        }
        pinmask
    }

    #[inline]
    pub fn under_attack_by_white(&self) -> BitBoard {
        let mut res = 0;
        for i in BitBoardIter(self.white_queens) {
            res |= queen_moves(i, !(self.occupied & !self.black_kings));
        }
        for i in BitBoardIter(self.white_rooks) {
            res |= rook_moves(i, !(self.occupied & !self.black_kings));
        }
        for i in BitBoardIter(self.white_bishops) {
            res |= bishop_moves(i, !(self.occupied & !self.black_kings));
        }
        for i in BitBoardIter(self.white_knights) {
            res |= KNIGHT_MOVES[i];
        }
        for i in BitBoardIter(self.white_kings) {
            res |= KING_MOVES[i];
        }
        for i in BitBoardIter(self.white_pawns) {
            res |= PAWN_ATTACKS[0][i];
        }
        res
    }
}
