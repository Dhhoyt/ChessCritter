use super::pseudomoves::*;
use super::utils::*;
use super::{Board, BoardMove, Piece};

impl Board {
    #[inline]
    pub fn white_moves(&self) -> Vec<(BoardMove, usize)> {
        let mut res: Vec<(BoardMove, usize)> = Vec::with_capacity(40);
        let checkmask = self.white_checkmask();
        let pinmask_d = self.white_pinmask_d();
        let pinmask_hv = self.white_pinmask_hv();
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
                };
                res.push((board_move, piece.1 + 2));
            }
        }

        //Unpinned Bishop Moves
        for i in BitBoardIter(self.white_bishops & !pinmask) {
            let moves = bishop_moves(i, !self.occupied) & checkmask;
            for m in BitBoardIter(moves & !self.occupied) {
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Bishop,
                    taken: Piece::None,
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
                };
                res.push((board_move, piece.1 + 1));
            }
        }
        let king_square = self.white_kings.trailing_zeros() as usize;
        let under_attack = self.under_attack_by_black();
        let moves = KING_MOVES[king_square] & !under_attack;
        for m in BitBoardIter(moves & !self.occupied) {
            let board_move = BoardMove {
                from: king_square,
                to: m,
                piece: Piece::King,
                taken: Piece::None,
                promotion: Piece::None,
                white: true,
                en_passant: false,
                last_castle: self.castle,
                last_en_passant: self.en_passant,
                last_occupied: self.occupied,
                last_white_occupied: self.white_occupied,
                last_black_occupied: self.black_occupied,
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
                promotion: Piece::None,
                white: true,
                en_passant: false,
                last_castle: self.castle,
                last_en_passant: self.en_passant,
                last_occupied: self.occupied,
                last_white_occupied: self.white_occupied,
                last_black_occupied: self.black_occupied,
            };
            res.push((board_move, piece.1));
        }

        //Pawns
        //Unpinned Pawn Pushes
        for i in BitBoardIter(self.white_pawns & !pinmask & south_one(!self.occupied)) {
            let moves = PAWN_MOVES[0][i] & checkmask & !self.occupied;
            for m in BitBoardIter(moves) {
                if m > 55 {
                    let board_move = BoardMove {
                        from: i,
                        to: m,
                        piece: Piece::Pawn,
                        taken: Piece::None,
                        promotion: Piece::Queen,
                        white: true,
                        en_passant: false,
                        last_castle: self.castle,
                        last_en_passant: self.en_passant,
                        last_occupied: self.occupied,
                        last_white_occupied: self.white_occupied,
                        last_black_occupied: self.black_occupied,
                    };
                    res.push((board_move, 1000));
                    let board_move = BoardMove {
                        from: i,
                        to: m,
                        piece: Piece::Pawn,
                        taken: Piece::None,
                        promotion: Piece::Rook,
                        white: true,
                        en_passant: false,
                        last_castle: self.castle,
                        last_en_passant: self.en_passant,
                        last_occupied: self.occupied,
                        last_white_occupied: self.white_occupied,
                        last_black_occupied: self.black_occupied,
                    };
                    res.push((board_move, 900));
                    let board_move = BoardMove {
                        from: i,
                        to: m,
                        piece: Piece::Pawn,
                        taken: Piece::None,
                        promotion: Piece::Bishop,
                        white: true,
                        en_passant: false,
                        last_castle: self.castle,
                        last_en_passant: self.en_passant,
                        last_occupied: self.occupied,
                        last_white_occupied: self.white_occupied,
                        last_black_occupied: self.black_occupied,
                    };
                    res.push((board_move, 800));
                    let board_move = BoardMove {
                        from: i,
                        to: m,
                        piece: Piece::Pawn,
                        taken: Piece::None,
                        promotion: Piece::Knight,
                        white: true,
                        en_passant: false,
                        last_castle: self.castle,
                        last_en_passant: self.en_passant,
                        last_occupied: self.occupied,
                        last_white_occupied: self.white_occupied,
                        last_black_occupied: self.black_occupied,
                    };
                    res.push((board_move, 700));
                } else {
                    let board_move = BoardMove {
                        from: i,
                        to: m,
                        piece: Piece::Pawn,
                        taken: Piece::None,
                        promotion: Piece::None,
                        white: true,
                        en_passant: false,
                        last_castle: self.castle,
                        last_en_passant: self.en_passant,
                        last_occupied: self.occupied,
                        last_white_occupied: self.white_occupied,
                        last_black_occupied: self.black_occupied,
                    };
                    res.push((board_move, 5));
                }
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
                    promotion: Piece::None,
                    white: true,
                    en_passant: false,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
                };
                res.push((board_move, 5));
            }
        }
        //Unpinned Pawn Attacks
        for i in BitBoardIter(self.white_pawns & !pinmask) {
            let moves = PAWN_ATTACKS[0][i] & checkmask & self.black_occupied;
            for m in BitBoardIter(moves) {
                let piece = self.what_black_piece(m);
                if m > 55 {
                    let board_move = BoardMove {
                        from: i,
                        to: m,
                        piece: Piece::Pawn,
                        taken: piece.0.clone(),
                        promotion: Piece::Queen,
                        white: true,
                        en_passant: false,
                        last_castle: self.castle,
                        last_en_passant: self.en_passant,
                        last_occupied: self.occupied,
                        last_white_occupied: self.white_occupied,
                        last_black_occupied: self.black_occupied,
                    };
                    res.push((board_move, piece.1 + 1000));
                    let board_move = BoardMove {
                        from: i,
                        to: m,
                        piece: Piece::Pawn,
                        taken: piece.0.clone(),
                        promotion: Piece::Rook,
                        white: true,
                        en_passant: false,
                        last_castle: self.castle,
                        last_en_passant: self.en_passant,
                        last_occupied: self.occupied,
                        last_white_occupied: self.white_occupied,
                        last_black_occupied: self.black_occupied,
                    };
                    res.push((board_move, piece.1 + 900));
                    let board_move = BoardMove {
                        from: i,
                        to: m,
                        piece: Piece::Pawn,
                        taken: piece.0.clone(),
                        promotion: Piece::Bishop,
                        white: true,
                        en_passant: false,
                        last_castle: self.castle,
                        last_en_passant: self.en_passant,
                        last_occupied: self.occupied,
                        last_white_occupied: self.white_occupied,
                        last_black_occupied: self.black_occupied,
                    };
                    res.push((board_move, piece.1 + 800));
                    let board_move = BoardMove {
                        from: i,
                        to: m,
                        piece: Piece::Pawn,
                        taken: piece.0,
                        promotion: Piece::Knight,
                        white: true,
                        en_passant: false,
                        last_castle: self.castle,
                        last_en_passant: self.en_passant,
                        last_occupied: self.occupied,
                        last_white_occupied: self.white_occupied,
                        last_black_occupied: self.black_occupied,
                    };
                    res.push((board_move, piece.1 + 700));
                } else {
                    let board_move = BoardMove {
                        from: i,
                        to: m,
                        piece: Piece::Pawn,
                        taken: piece.0,
                        promotion: Piece::None,
                        white: true,
                        en_passant: false,
                        last_castle: self.castle,
                        last_en_passant: self.en_passant,
                        last_occupied: self.occupied,
                        last_white_occupied: self.white_occupied,
                        last_black_occupied: self.black_occupied,
                    };
                    res.push((board_move, piece.1 + 5));
                }
            }
        }
        //Pinned Pawn Attacks
        for i in BitBoardIter(self.white_pawns & pinmask) {
            let moves = PAWN_ATTACKS[0][i] & checkmask & self.black_occupied & pinmask_d;
            for m in BitBoardIter(moves) {
                let piece = self.what_black_piece(m);
                if m > 55 {
                    let board_move = BoardMove {
                        from: i,
                        to: m,
                        piece: Piece::Pawn,
                        taken: piece.0.clone(),
                        promotion: Piece::Queen,
                        white: true,
                        en_passant: false,
                        last_castle: self.castle,
                        last_en_passant: self.en_passant,
                        last_occupied: self.occupied,
                        last_white_occupied: self.white_occupied,
                        last_black_occupied: self.black_occupied,
                    };
                    res.push((board_move, piece.1 + 1000));
                    let board_move = BoardMove {
                        from: i,
                        to: m,
                        piece: Piece::Pawn,
                        taken: piece.0.clone(),
                        promotion: Piece::Rook,
                        white: true,
                        en_passant: false,
                        last_castle: self.castle,
                        last_en_passant: self.en_passant,
                        last_occupied: self.occupied,
                        last_white_occupied: self.white_occupied,
                        last_black_occupied: self.black_occupied,
                    };
                    res.push((board_move, piece.1 + 900));
                    let board_move = BoardMove {
                        from: i,
                        to: m,
                        piece: Piece::Pawn,
                        taken: piece.0.clone(),
                        promotion: Piece::Bishop,
                        white: true,
                        en_passant: false,
                        last_castle: self.castle,
                        last_en_passant: self.en_passant,
                        last_occupied: self.occupied,
                        last_white_occupied: self.white_occupied,
                        last_black_occupied: self.black_occupied,
                    };
                    res.push((board_move, piece.1 + 800));
                    let board_move = BoardMove {
                        from: i,
                        to: m,
                        piece: Piece::Pawn,
                        taken: piece.0,
                        promotion: Piece::Knight,
                        white: true,
                        en_passant: false,
                        last_castle: self.castle,
                        last_en_passant: self.en_passant,
                        last_occupied: self.occupied,
                        last_white_occupied: self.white_occupied,
                        last_black_occupied: self.black_occupied,
                    };
                    res.push((board_move, piece.1 + 700));
                } else {
                    let board_move = BoardMove {
                        from: i,
                        to: m,
                        piece: Piece::Pawn,
                        taken: piece.0,
                        promotion: Piece::None,
                        white: true,
                        en_passant: false,
                        last_castle: self.castle,
                        last_en_passant: self.en_passant,
                        last_occupied: self.occupied,
                        last_white_occupied: self.white_occupied,
                        last_black_occupied: self.black_occupied,
                    };
                    res.push((board_move, piece.1 + 5));
                }
            }
        }
        //Unpinned En Passant
        for i in BitBoardIter(self.white_pawns & !pinmask) {
            let moves = PAWN_ATTACKS[0][i] & checkmask & self.en_passant;
            for m in BitBoardIter(moves) {
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Pawn,
                    taken: Piece::Pawn,
                    promotion: Piece::None,
                    white: true,
                    en_passant: true,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
                };
                res.push((board_move, 105));
            }
        }
        //Pinned En Passant
        for i in BitBoardIter(self.white_pawns & pinmask) {
            let moves = PAWN_ATTACKS[0][i] & checkmask & self.en_passant & pinmask_d;
            for m in BitBoardIter(moves) {
                let board_move = BoardMove {
                    from: i,
                    to: m,
                    piece: Piece::Pawn,
                    taken: Piece::Pawn,
                    promotion: Piece::None,
                    white: true,
                    en_passant: true,
                    last_castle: self.castle,
                    last_en_passant: self.en_passant,
                    last_occupied: self.occupied,
                    last_white_occupied: self.white_occupied,
                    last_black_occupied: self.black_occupied,
                };
                res.push((board_move, 105));
            }
        }

        //Castleing
        if (self.castle & 0x90 == 0x90) && under_attack & 0x70 == 0 && self.occupied & 0x60 == 0 {
            let board_move = BoardMove {
                from: 4,
                to: 7,
                piece: Piece::King,
                taken: Piece::None,
                promotion: Piece::None,
                white: true,
                en_passant: false,
                last_castle: self.castle,
                last_en_passant: self.en_passant,
                last_occupied: self.occupied,
                last_white_occupied: self.white_occupied,
                last_black_occupied: self.black_occupied,
            };
            res.push((board_move, 0));
        }

        if (self.castle & 0x11 == 0x11) && under_attack & 0x1c == 0 && self.occupied & 0xe == 0 {
            let board_move = BoardMove {
                from: 4,
                to: 0,
                piece: Piece::King,
                taken: Piece::None,
                promotion: Piece::None,
                white: true,
                en_passant: false,
                last_castle: self.castle,
                last_en_passant: self.en_passant,
                last_occupied: self.occupied,
                last_white_occupied: self.white_occupied,
                last_black_occupied: self.black_occupied,
            };
            res.push((board_move, 0));
        }
        res
    }
}
