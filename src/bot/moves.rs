use super::pseudomoves::*;
use super::utils::*;
use super::{BitBoard, Board, BoardMove, Piece};

impl Board {
    #[inline]
    fn white_moves(&self) -> Vec<(BoardMove, usize)> {
        let mut res: Vec<(BoardMove, usize)> = Vec::with_capacity(40);
        let checkmask = self.black_checkmask();
        let pinmask_d = self.black_pinmask_d();
        let pinmask_hv = self.black_pinmask_hv();
        let pinmask = pinmask_d | pinmask_hv;
        //Unpinned rook moves
        for i in BitBoardIter(self.white_rooks & !pinmask) {
            let moves = rook_moves(i, !self.occupied) & checkmask;
            for m in BitBoardIter(moves & !self.occupied) {
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Rook,
                    taken: Piece::None,
                    white: true,
                };
                res.push((board_move, 2));
            }
            for m in BitBoardIter(moves & self.black_occupied) {
                let piece = self.what_black_piece(m);
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Rook,
                    taken: piece.0,
                    white: true,
                };
                res.push((board_move, piece.1 + 2));
            }
        }
        for i in BitBoardIter(self.white_rooks & pinmask) {
            let moves = rook_moves(i, !self.occupied) & checkmask & pinmask_hv;
            for m in BitBoardIter(moves & !self.occupied) {
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Rook,
                    taken: Piece::None,
                    white: true,
                };
                res.push((board_move, 2));
            }
            for m in BitBoardIter(moves & self.black_occupied) {
                let piece = self.what_black_piece(m);
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Rook,
                    taken: piece.0,
                    white: true,
                };
                res.push((board_move, piece.1 + 2));
            }
        }

        //Unpinned Bishops
        for i in BitBoardIter(self.white_bishops & !pinmask) {
            let moves = bishop_moves(i, !self.occupied) & checkmask;
            for m in BitBoardIter(moves & !self.occupied) {
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Bishop,
                    taken: Piece::None,
                    white: true,
                };
                res.push((board_move, 3));
            }
            for m in BitBoardIter(moves & self.black_occupied) {
                let piece = self.what_black_piece(m);
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Bishop,
                    taken: piece.0,
                    white: true,
                };
                res.push((board_move, piece.1 + 3));
            }
        }
        //Pinned bishop moves
        for i in BitBoardIter(self.white_bishops & pinmask) {
            let moves = bishop_moves(i, !self.occupied) & checkmask & pinmask_d;
            for m in BitBoardIter(moves & !self.occupied) {
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Bishop,
                    taken: Piece::None,
                    white: true,
                };
                res.push((board_move, 3));
            }
            for m in BitBoardIter(moves & self.black_occupied) {
                let piece = self.what_black_piece(m);
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Bishop,
                    taken: piece.0,
                    white: true,
                };
                res.push((board_move, piece.1 + 3));
            }
        }

        //A pinned knight can't move
        for i in BitBoardIter(self.white_knights & !pinmask) {
            let moves = KNIGHT_MOVES[i] & checkmask;
            for m in BitBoardIter(moves & !self.occupied) {
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Knight,
                    taken: Piece::None,
                    white: true,
                };
                res.push((board_move, 4));
            }
            for m in BitBoardIter(moves & self.black_occupied) {
                let piece = self.what_black_piece(m);
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Knight,
                    taken: piece.0,
                    white: true,
                };
                res.push((board_move, piece.1 + 4));
            }
        }

        // Queen moves
        // Unpinned queen rook moves
        for i in BitBoardIter(self.white_queens & !pinmask) {
            let moves = rook_moves(i, !self.occupied) & checkmask;
            for m in BitBoardIter(moves & !self.occupied) {
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Queen,
                    taken: Piece::None,
                    white: true,
                };
                res.push((board_move, 1));
            }
            for m in BitBoardIter(moves & self.black_occupied) {
                let piece = self.what_black_piece(m);
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Queen,
                    taken: piece.0,
                    white: true,
                };
                res.push((board_move, piece.1 + 1));
            }
        }
        //Pinned queen rook moves
        for i in BitBoardIter(self.white_queens & pinmask) {
            let moves = rook_moves(i, !self.occupied) & checkmask & pinmask_hv;
            for m in BitBoardIter(moves & !self.occupied) {
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Queen,
                    taken: Piece::None,
                    white: true,
                };
                res.push((board_move, 1));
            }
            for m in BitBoardIter(moves & self.black_occupied) {
                let piece = self.what_black_piece(m);
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Queen,
                    taken: piece.0,
                    white: true,
                };
                res.push((board_move, piece.1 + 1));
            }
        }
        //Unpinned queen Bishops
        for i in BitBoardIter(self.white_queens & !pinmask) {
            let moves = bishop_moves(i, !self.occupied) & checkmask;
            for m in BitBoardIter(moves & !self.occupied) {
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Queen,
                    taken: Piece::None,
                    white: true,
                };
                res.push((board_move, 1));
            }
            for m in BitBoardIter(moves & self.black_occupied) {
                let piece = self.what_black_piece(m);
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Queen,
                    taken: piece.0,
                    white: true,
                };
                res.push((board_move, piece.1 + 1));
            }
        }
        //Pinned queen bishop moves
        for i in BitBoardIter(self.white_queens & pinmask) {
            let moves = bishop_moves(i, !self.occupied) & checkmask & pinmask_d;
            for m in BitBoardIter(moves & !self.occupied) {
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Queen,
                    taken: Piece::None,
                    white: true,
                };
                res.push((board_move, 1));
            }
            for m in BitBoardIter(moves & self.black_occupied) {
                let piece = self.what_black_piece(m);
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Queen,
                    taken: piece.0,
                    white: true,
                };
                res.push((board_move, piece.1 + 1));
            }
        }
        let king_square = self.white_kings.trailing_zeros() as usize;
        let under_attack = self.under_attack_by_white();
        let moves = KING_MOVES[king_square] & !under_attack;
        for m in BitBoardIter(moves & !self.occupied) {
            let board_move = BoardMove {
                from: king_square,
                to: m,
                piece: Piece::King,
                taken: Piece::None,
                white: true,
            };
            res.push((board_move, 0));
        }
        for m in BitBoardIter(moves & self.black_occupied) {
            let piece = self.what_black_piece(m);
            let board_move = BoardMove {
                from: king_square,
                to: m,
                piece: Piece::King,
                taken: piece.0,
                white: true,
            };
            res.push((board_move, piece.1));
        }

        //Pawns
        //Unpinned Pawn Pushes
        for i in BitBoardIter(self.white_pawns & !pinmask & south_one(!self.occupied)) {
            let moves = PAWN_MOVES[0][i] & checkmask & !self.occupied;
            for m in BitBoardIter(moves) {
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Pawn,
                    taken: Piece::None,
                    white: true,
                };
                res.push((board_move, 5));
            }
        }
        //Pinned pawn pushes
        for i in BitBoardIter(self.white_pawns & pinmask & south_one(!self.occupied)) {
            let moves = PAWN_MOVES[0][i] & checkmask & !self.occupied & pinmask_hv;
            for m in BitBoardIter(moves) {
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Pawn,
                    taken: Piece::None,
                    white: true,
                };
                res.push((board_move, 5));
            }
        }
        res
    }
}
