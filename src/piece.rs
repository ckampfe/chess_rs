use crate::board::Board;
use crate::position::{MoveDirection, Position, XY};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

pub trait Piece: Display {
    fn color(&self) -> Color;
    fn position(&self) -> Position;
    fn moves(&self, board: &Board) -> HashSet<Position>;
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

#[derive(Clone, Debug)]
pub struct Pawn {
    color: Color,
    position: Position,
}

impl Pawn {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Piece for Pawn {
    fn color(&self) -> Color {
        self.color
    }
    fn position(&self) -> Position {
        self.position
    }

    // TODO: add check for en passant
    fn moves(&self, board: &Board) -> HashSet<Position> {
        let same_color_piece_positions = board
            .pieces()
            .iter()
            .filter(|piece| piece.color() == self.color())
            .map(|piece| piece.position())
            .collect::<HashSet<Position>>();

        let opposite_color_piece_positions = board
            .pieces()
            .iter()
            .filter(|piece| piece.color() != self.color())
            .map(|piece| piece.position())
            .collect::<HashSet<Position>>();

        let all_piece_positions: HashSet<Position> = same_color_piece_positions
            .union(&opposite_color_piece_positions)
            .copied()
            .collect();

        let in_home_row = matches!(
            (self.color(), self.position().to_xy()),
            (Color::White, XY::OnBoard(_, 1)) | (Color::Black, XY::OnBoard(_, 6))
        );

        let mut all = HashSet::new();

        match self.color() {
            Color::Black => {
                if !all_piece_positions.contains(&self.position().down()) {
                    all.insert(self.position().down());
                }

                for position in [self.position().down_left(), self.position().down_right()] {
                    if opposite_color_piece_positions.contains(&position) {
                        all.insert(position);
                    }
                }

                if in_home_row
                    && !all_piece_positions.contains(&self.position().down().down())
                    && !all_piece_positions.contains(&self.position().down())
                {
                    all.insert(self.position().down().down());
                }
            }
            Color::White => {
                if !all_piece_positions.contains(&self.position().up()) {
                    all.insert(self.position().up());
                }

                for position in [self.position().up_left(), self.position().up_right()] {
                    if opposite_color_piece_positions.contains(&position) {
                        all.insert(position);
                    }
                }

                if in_home_row
                    && !all_piece_positions.contains(&self.position().up().up())
                    && !all_piece_positions.contains(&self.position().up())
                {
                    all.insert(self.position().up().up());
                }
            }
        };

        all.iter()
            .filter(|position| !same_color_piece_positions.contains(position))
            .copied()
            .collect()
    }
}

impl Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self.color() {
            Color::Black => "\u{265F}",
            Color::White => "\u{2659}",
        };

        write!(f, "{}", out)
    }
}

#[derive(Clone, Debug)]
pub struct Knight {
    color: Color,
    position: Position,
}

impl Knight {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Piece for Knight {
    fn color(&self) -> Color {
        self.color
    }

    fn position(&self) -> Position {
        self.position
    }

    fn moves(&self, board: &Board) -> HashSet<Position> {
        let same_color_piece_positions = board
            .pieces()
            .iter()
            .filter(|piece| piece.color() == self.color())
            .map(|piece| piece.position())
            .collect::<HashSet<Position>>();

        [
            Position::compose([MoveDirection::Up, MoveDirection::Up, MoveDirection::Right]),
            Position::compose([MoveDirection::Up, MoveDirection::Up, MoveDirection::Left]),
            Position::compose([
                MoveDirection::Right,
                MoveDirection::Right,
                MoveDirection::Up,
            ]),
            Position::compose([
                MoveDirection::Right,
                MoveDirection::Right,
                MoveDirection::Down,
            ]),
            Position::compose([
                MoveDirection::Down,
                MoveDirection::Down,
                MoveDirection::Right,
            ]),
            Position::compose([
                MoveDirection::Down,
                MoveDirection::Down,
                MoveDirection::Left,
            ]),
            Position::compose([
                MoveDirection::Left,
                MoveDirection::Left,
                MoveDirection::Down,
            ]),
            Position::compose([MoveDirection::Left, MoveDirection::Left, MoveDirection::Up]),
        ]
        .iter()
        .map(|this_move| this_move(self.position()))
        .filter(|position| position.is_on_board())
        .filter(|position| !same_color_piece_positions.contains(position))
        .collect()
    }
}

impl Display for Knight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self.color() {
            Color::Black => "\u{265E}",
            Color::White => "\u{2658}",
        };

        write!(f, "{}", out)
    }
}

#[derive(Clone, Debug)]
pub struct Bishop {
    color: Color,
    position: Position,
}

impl Bishop {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Piece for Bishop {
    fn color(&self) -> Color {
        self.color
    }

    fn position(&self) -> Position {
        self.position
    }

    fn moves(&self, board: &Board) -> HashSet<Position> {
        let pieces_map: HashMap<Position, &Box<dyn Piece>> = board
            .pieces()
            .iter()
            .map(|piece| (piece.position(), piece))
            .collect();

        let mut moves = HashSet::new();

        for move_direction in [
            MoveDirection::UpLeft,
            MoveDirection::UpRight,
            MoveDirection::DownRight,
            MoveDirection::DownLeft,
        ] {
            for (position, maybe_piece) in self
                .position()
                .stream(move_direction)
                .take_while(|position| position.is_on_board())
                .map(|position| (position, pieces_map.get(&position)))
            {
                if let Some(piece) = maybe_piece {
                    if piece.color() == self.color() {
                    } else {
                        moves.insert(position);
                    }

                    break;
                } else {
                    moves.insert(position);
                }
            }
        }

        moves
    }
}

impl Display for Bishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self.color() {
            Color::Black => "\u{265D}",
            Color::White => "\u{2657}",
        };

        write!(f, "{}", out)
    }
}

#[derive(Clone, Debug)]
pub struct Rook {
    color: Color,
    position: Position,
}

impl Rook {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Piece for Rook {
    fn color(&self) -> Color {
        self.color
    }

    fn position(&self) -> Position {
        self.position
    }

    fn moves(&self, board: &Board) -> HashSet<Position> {
        let pieces_map: HashMap<Position, &Box<dyn Piece>> = board
            .pieces()
            .iter()
            .map(|piece| (piece.position(), piece))
            .collect();

        let mut moves = HashSet::new();

        for move_direction in [
            MoveDirection::Up,
            MoveDirection::Right,
            MoveDirection::Down,
            MoveDirection::Left,
        ] {
            for (position, maybe_piece) in self
                .position()
                .stream(move_direction)
                .take_while(|position| position.is_on_board())
                .map(|position| (position, pieces_map.get(&position)))
            {
                if let Some(piece) = maybe_piece {
                    if piece.color() == self.color() {
                    } else {
                        moves.insert(position);
                    }

                    break;
                } else {
                    moves.insert(position);
                }
            }
        }

        moves
    }
}

impl Display for Rook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self.color() {
            Color::Black => "\u{265C}",
            Color::White => "\u{2656}",
        };

        write!(f, "{}", out)
    }
}

#[derive(Clone, Debug)]
pub struct Queen {
    color: Color,
    position: Position,
}

impl Queen {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Piece for Queen {
    fn color(&self) -> Color {
        self.color
    }

    fn position(&self) -> Position {
        self.position
    }

    fn moves(&self, board: &Board) -> HashSet<Position> {
        let pieces_map: HashMap<Position, &Box<dyn Piece>> = board
            .pieces()
            .iter()
            .map(|piece| (piece.position(), piece))
            .collect();

        let mut moves = HashSet::new();

        for move_direction in [
            MoveDirection::Up,
            MoveDirection::Right,
            MoveDirection::Down,
            MoveDirection::Left,
            MoveDirection::UpLeft,
            MoveDirection::UpRight,
            MoveDirection::DownRight,
            MoveDirection::DownLeft,
        ] {
            for (position, maybe_piece) in self
                .position()
                .stream(move_direction)
                .take_while(|position| position.is_on_board())
                .map(|position| (position, pieces_map.get(&position)))
            {
                if let Some(piece) = maybe_piece {
                    if piece.color() == self.color() {
                    } else {
                        moves.insert(position);
                    }

                    break;
                } else {
                    moves.insert(position);
                }
            }
        }

        moves
    }
}

impl Display for Queen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self.color() {
            Color::Black => "\u{265B}",
            Color::White => "\u{2655}",
        };

        write!(f, "{}", out)
    }
}

#[derive(Clone, Debug)]
pub struct King {
    color: Color,
    position: Position,
}

impl King {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Piece for King {
    fn color(&self) -> Color {
        self.color
    }

    fn position(&self) -> Position {
        self.position
    }

    // TODO: add check for moving into check
    // TODO: add castling
    fn moves(&self, board: &Board) -> HashSet<Position> {
        let same_color_piece_positions = board
            .pieces()
            .iter()
            .filter(|piece| piece.color() == self.color())
            .map(|piece| piece.position())
            .collect::<HashSet<Position>>();

        [
            self.position().up(),
            self.position().up_right(),
            self.position().right(),
            self.position().down_right(),
            self.position().down(),
            self.position().down_left(),
            self.position().left(),
            self.position().up_left(),
        ]
        .into_iter()
        .filter(|position| position.is_on_board())
        .filter(|position| !same_color_piece_positions.contains(position))
        .collect()
    }
}

impl Display for King {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self.color() {
            Color::Black => "\u{265A}",
            Color::White => "\u{2654}",
        };

        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let expected_black = ["♟", "♞", "♝", "♜", "♛", "♚"];
        let expected_white = ["♙", "♘", "♗", "♖", "♕", "♔"];

        let pawn: Pawn = Pawn::new(Color::Black, Position::new(0, 0));
        let knight: Knight = Knight::new(Color::Black, Position::new(0, 0));
        let bishop: Bishop = Bishop::new(Color::Black, Position::new(0, 0));
        let rook: Rook = Rook::new(Color::Black, Position::new(0, 0));
        let queen: Queen = Queen::new(Color::Black, Position::new(0, 0));
        let king: King = King::new(Color::Black, Position::new(0, 0));

        let pieces_black: [Box<dyn Piece>; 6] = [
            Box::new(pawn),
            Box::new(knight),
            Box::new(bishop),
            Box::new(rook),
            Box::new(queen),
            Box::new(king),
        ];

        for (piece, expected_piece_string) in pieces_black.iter().zip(expected_black) {
            assert_eq!(piece.to_string(), expected_piece_string);
        }

        let pawn: Pawn = Pawn::new(Color::White, Position::new(0, 0));
        let knight: Knight = Knight::new(Color::White, Position::new(0, 0));
        let bishop: Bishop = Bishop::new(Color::White, Position::new(0, 0));
        let rook: Rook = Rook::new(Color::White, Position::new(0, 0));
        let queen: Queen = Queen::new(Color::White, Position::new(0, 0));
        let king: King = King::new(Color::White, Position::new(0, 0));

        let pieces_white: [Box<dyn Piece>; 6] = [
            Box::new(pawn),
            Box::new(knight),
            Box::new(bishop),
            Box::new(rook),
            Box::new(queen),
            Box::new(king),
        ];

        for (piece, expected_piece_string) in pieces_white.iter().zip(expected_white) {
            assert_eq!(piece.to_string(), expected_piece_string);
        }
    }

    mod pawn {
        use super::*;

        #[test]
        fn free() {
            let board = Board::empty();
            let pawn = Pawn::new(Color::Black, (4, 6).into());
            assert_eq!(
                pawn.moves(&board),
                HashSet::from([(4, 5).into(), (4, 4).into()])
            );

            let pawn = Pawn::new(Color::White, (4, 6).into());
            assert_eq!(pawn.moves(&board), HashSet::from([(4, 7).into()]))
        }

        #[test]
        fn blocks() {
            let board = Board::new(vec![Box::new(Pawn::new(Color::Black, (4, 5).into()))]);
            let pawn = Pawn::new(Color::Black, (4, 6).into());
            assert_eq!(pawn.moves(&board), HashSet::new());

            let board = Board::new(vec![Box::new(Pawn::new(Color::White, (5, 7).into()))]);
            let pawn = Pawn::new(Color::White, (4, 6).into());
            assert_eq!(pawn.moves(&board), HashSet::from([(4, 7).into()]));

            let board = Board::new(vec![Box::new(Pawn::new(Color::White, (4, 3).into()))]);
            let pawn = Pawn::new(Color::White, (4, 1).into());
            assert_eq!(pawn.moves(&board), HashSet::from([(4, 2).into()]));
        }

        #[test]
        fn takes() {
            let board = Board::new(vec![Box::new(Pawn::new(Color::White, (4, 5).into()))]);
            let pawn = Pawn::new(Color::Black, (4, 6).into());
            assert_eq!(pawn.moves(&board), HashSet::new());

            let board = Board::new(vec![Box::new(Pawn::new(Color::Black, (5, 7).into()))]);
            let pawn = Pawn::new(Color::White, (4, 6).into());
            assert_eq!(
                pawn.moves(&board),
                HashSet::from([(4, 7).into(), (5, 7).into()])
            );
        }
    }

    mod knight {
        use super::*;

        #[test]
        fn free() {
            let board = Board::empty();
            let knight = Knight::new(Color::Black, Position::new(4, 4));
            assert_eq!(
                knight.moves(&board).into_iter().collect::<HashSet<_>>(),
                HashSet::from([
                    // up
                    (5, 6).into(),
                    (3, 6).into(),
                    // right
                    (6, 5).into(),
                    (6, 3).into(),
                    // down
                    (3, 2).into(),
                    (5, 2).into(),
                    // left
                    (2, 5).into(),
                    (2, 3).into(),
                ])
            )
        }

        #[test]
        fn blocks() {
            let board = Board::new(vec![
                Box::new(Pawn::new(Color::Black, Position::new(6, 5))),
                Box::new(Pawn::new(Color::Black, Position::new(2, 3))),
                Box::new(Pawn::new(Color::White, Position::new(2, 5))),
            ]);

            let knight = Knight::new(Color::Black, Position::new(4, 4));

            assert_eq!(
                knight.moves(&board).into_iter().collect::<HashSet<_>>(),
                HashSet::from([
                    (5, 6).into(),
                    (3, 6).into(),
                    (6, 3).into(),
                    (3, 2).into(),
                    (5, 2).into(),
                    (2, 5).into(),
                ])
            )
        }

        #[test]
        fn takes() {
            let board = Board::new(vec![
                Box::new(Pawn::new(Color::White, Position::new(6, 5))),
                Box::new(Pawn::new(Color::White, Position::new(2, 3))),
                Box::new(Pawn::new(Color::White, Position::new(2, 5))),
            ]);

            let knight = Knight::new(Color::Black, Position::new(4, 4));

            assert_eq!(
                knight.moves(&board).into_iter().collect::<HashSet<_>>(),
                HashSet::from([
                    (5, 6).into(),
                    (3, 6).into(),
                    (6, 5).into(),
                    (6, 3).into(),
                    (3, 2).into(),
                    (5, 2).into(),
                    (2, 5).into(),
                    (2, 3).into(),
                ])
            )
        }
    }

    mod bishop {
        use super::*;

        #[test]
        fn free() {
            let board = Board::empty();

            let bishop = Bishop::new(Color::Black, Position::new(4, 4));

            assert_eq!(
                bishop.moves(&board).into_iter().collect::<HashSet<_>>(),
                HashSet::from([
                    (5, 5).into(),
                    (6, 6).into(),
                    (7, 7).into(),
                    //
                    (5, 3).into(),
                    (6, 2).into(),
                    (7, 1).into(),
                    //
                    (3, 3).into(),
                    (2, 2).into(),
                    (1, 1).into(),
                    (0, 0).into(),
                    //
                    (3, 5).into(),
                    (2, 6).into(),
                    (1, 7).into()
                ])
            )
        }

        #[test]
        fn blocks() {
            let board = Board::new(vec![
                Box::new(Pawn::new(Color::Black, Position::new(1, 7))),
                Box::new(Pawn::new(Color::Black, Position::new(6, 2))),
            ]);

            let bishop = Bishop::new(Color::Black, Position::new(4, 4));

            assert_eq!(
                bishop.moves(&board).into_iter().collect::<HashSet<_>>(),
                HashSet::from([
                    (5, 5).into(),
                    (6, 6).into(),
                    (7, 7).into(),
                    //
                    (5, 3).into(),
                    //
                    (3, 3).into(),
                    (2, 2).into(),
                    (1, 1).into(),
                    (0, 0).into(),
                    //
                    (3, 5).into(),
                    (2, 6).into(),
                ])
            )
        }

        #[test]
        fn takes() {
            let board = Board::new(vec![
                Box::new(Pawn::new(Color::White, Position::new(1, 7))),
                Box::new(Pawn::new(Color::White, Position::new(6, 2))),
            ]);

            let bishop = Bishop::new(Color::Black, Position::new(4, 4));

            assert_eq!(
                bishop.moves(&board).into_iter().collect::<HashSet<_>>(),
                HashSet::from([
                    (5, 5).into(),
                    (6, 6).into(),
                    (7, 7).into(),
                    //
                    (5, 3).into(),
                    (6, 2).into(),
                    //
                    (3, 3).into(),
                    (2, 2).into(),
                    (1, 1).into(),
                    (0, 0).into(),
                    //
                    (3, 5).into(),
                    (2, 6).into(),
                    (1, 7).into(),
                ])
            )
        }
    }

    mod rook {
        use super::*;

        #[test]
        fn free() {
            let board = Board::empty();
            let rook = Rook::new(Color::Black, Position::new(4, 4));

            assert_eq!(
                rook.moves(&board).into_iter().collect::<HashSet<_>>(),
                HashSet::from([
                    // up
                    (4, 5).into(),
                    (4, 6).into(),
                    (4, 7).into(),
                    // right
                    (5, 4).into(),
                    (6, 4).into(),
                    (7, 4).into(),
                    // down
                    (4, 3).into(),
                    (4, 2).into(),
                    (4, 1).into(),
                    (4, 0).into(),
                    // left
                    (3, 4).into(),
                    (2, 4).into(),
                    (1, 4).into(),
                    (0, 4).into(),
                ])
            )
        }

        #[test]
        fn blocks() {
            let board = Board::new(vec![
                Box::new(Pawn::new(Color::Black, Position::new(4, 5))),
                Box::new(Pawn::new(Color::Black, Position::new(4, 3))),
            ]);
            let rook = Rook::new(Color::Black, Position::new(4, 4));

            assert_eq!(
                rook.moves(&board).into_iter().collect::<HashSet<_>>(),
                HashSet::from([
                    // up
                    // right
                    (5, 4).into(),
                    (6, 4).into(),
                    (7, 4).into(),
                    // down
                    // left
                    (3, 4).into(),
                    (2, 4).into(),
                    (1, 4).into(),
                    (0, 4).into(),
                ])
            )
        }

        #[test]
        fn takes() {
            let board = Board::new(vec![
                Box::new(Pawn::new(Color::Black, Position::new(4, 5))),
                Box::new(Pawn::new(Color::Black, Position::new(4, 3))),
                Box::new(Pawn::new(Color::White, Position::new(1, 4))),
                Box::new(Pawn::new(Color::White, Position::new(6, 4))),
            ]);
            let rook = Rook::new(Color::Black, Position::new(4, 4));

            assert_eq!(
                rook.moves(&board).into_iter().collect::<HashSet<_>>(),
                HashSet::from([
                    // up
                    // right
                    (5, 4).into(),
                    (6, 4).into(),
                    // down
                    // left
                    (3, 4).into(),
                    (2, 4).into(),
                    (1, 4).into(),
                ])
            )
        }
    }

    mod queen {
        use super::*;

        #[test]
        fn free() {
            let board = Board::empty();
            let queen = Queen::new(Color::Black, (4, 4).into());
            assert_eq!(
                queen.moves(&board),
                HashSet::from([
                    (4, 5).into(),
                    (4, 6).into(),
                    (4, 7).into(),
                    // right
                    (5, 4).into(),
                    (6, 4).into(),
                    (7, 4).into(),
                    // down
                    (4, 3).into(),
                    (4, 2).into(),
                    (4, 1).into(),
                    (4, 0).into(),
                    // left
                    (3, 4).into(),
                    (2, 4).into(),
                    (1, 4).into(),
                    (0, 4).into(),
                    // up right
                    (5, 5).into(),
                    (6, 6).into(),
                    (7, 7).into(),
                    // down right
                    (5, 3).into(),
                    (6, 2).into(),
                    (7, 1).into(),
                    // down left
                    (3, 3).into(),
                    (2, 2).into(),
                    (1, 1).into(),
                    (0, 0).into(),
                    // up left
                    (3, 5).into(),
                    (2, 6).into(),
                    (1, 7).into()
                ])
            )
        }

        #[test]
        fn blocks() {
            let board = Board::new(vec![Box::new(Pawn::new(Color::Black, (4, 5).into()))]);
            let queen = Queen::new(Color::Black, (4, 4).into());
            assert_eq!(
                queen.moves(&board),
                HashSet::from([
                    // up
                    // right
                    (5, 4).into(),
                    (6, 4).into(),
                    (7, 4).into(),
                    // down
                    (4, 3).into(),
                    (4, 2).into(),
                    (4, 1).into(),
                    (4, 0).into(),
                    // left
                    (3, 4).into(),
                    (2, 4).into(),
                    (1, 4).into(),
                    (0, 4).into(),
                    // up right
                    (5, 5).into(),
                    (6, 6).into(),
                    (7, 7).into(),
                    // down right
                    (5, 3).into(),
                    (6, 2).into(),
                    (7, 1).into(),
                    // down left
                    (3, 3).into(),
                    (2, 2).into(),
                    (1, 1).into(),
                    (0, 0).into(),
                    // up left
                    (3, 5).into(),
                    (2, 6).into(),
                    (1, 7).into()
                ])
            )
        }

        #[test]
        fn takes() {
            let board = Board::new(vec![
                Box::new(Pawn::new(Color::Black, (4, 5).into())),
                Box::new(Pawn::new(Color::White, (2, 2).into())),
            ]);

            let queen = Queen::new(Color::Black, (4, 4).into());

            assert_eq!(
                queen.moves(&board),
                HashSet::from([
                    // up
                    // right
                    (5, 4).into(),
                    (6, 4).into(),
                    (7, 4).into(),
                    // down
                    (4, 3).into(),
                    (4, 2).into(),
                    (4, 1).into(),
                    (4, 0).into(),
                    // left
                    (3, 4).into(),
                    (2, 4).into(),
                    (1, 4).into(),
                    (0, 4).into(),
                    // up right
                    (5, 5).into(),
                    (6, 6).into(),
                    (7, 7).into(),
                    // down right
                    (5, 3).into(),
                    (6, 2).into(),
                    (7, 1).into(),
                    // down left
                    (3, 3).into(),
                    (2, 2).into(),
                    // up left
                    (3, 5).into(),
                    (2, 6).into(),
                    (1, 7).into()
                ])
            )
        }
    }
    mod king {
        use super::*;
        use crate::piece::Piece;

        #[test]
        fn free() {
            let board = Board::empty();
            let king = King::new(Color::Black, Position::new(4, 4));
            assert_eq!(
                king.moves(&board),
                HashSet::from([
                    (4, 5).into(),
                    (5, 5).into(),
                    (5, 4).into(),
                    (5, 3).into(),
                    (4, 3).into(),
                    (3, 3).into(),
                    (3, 4).into(),
                    (3, 5).into()
                ])
            )
        }

        #[test]
        fn blocks() {
            let board = Board::new(vec![
                Box::new(Pawn::new(Color::Black, (4, 5).into())),
                Box::new(Pawn::new(Color::Black, (3, 3).into())),
            ]);

            let king = King::new(Color::Black, (4, 4).into());

            assert_eq!(
                king.moves(&board),
                HashSet::from([
                    (5, 5).into(),
                    (5, 4).into(),
                    (5, 3).into(),
                    (4, 3).into(),
                    (3, 4).into(),
                    (3, 5).into()
                ])
            )
        }

        #[test]
        fn takes() {
            let board = Board::new(vec![
                Box::new(Pawn::new(Color::White, (4, 5).into())),
                Box::new(Pawn::new(Color::White, (3, 3).into())),
            ]);

            let king = King::new(Color::Black, (4, 4).into());

            assert_eq!(
                king.moves(&board),
                HashSet::from([
                    (4, 5).into(),
                    (5, 5).into(),
                    (5, 4).into(),
                    (5, 3).into(),
                    (4, 3).into(),
                    (3, 3).into(),
                    (3, 4).into(),
                    (3, 5).into()
                ])
            )
        }
    }
}
