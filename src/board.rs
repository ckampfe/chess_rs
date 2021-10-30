use std::collections::HashMap;
use std::fmt::Display;

use crate::piece::{Bishop, Color, King, Knight, Pawn, Piece, Queen, Rook};
use crate::position::{Position, XY};

pub struct Board {
    pieces: Vec<Box<dyn Piece>>,
}

impl Board {
    pub fn new(pieces: Vec<Box<dyn Piece>>) -> Self {
        Board { pieces }
    }

    pub fn empty() -> Self {
        Board { pieces: vec![] }
    }

    pub fn pieces(&self) -> &[Box<dyn Piece>] {
        &self.pieces
    }
}

impl Default for Board {
    fn default() -> Self {
        let pawns = HashMap::from([(Color::White, 1), (Color::Black, 6)])
            .into_iter()
            .flat_map(|(color, y)| {
                (0..8)
                    .map(|x| Box::new(Pawn::new(color, Position::new(x, y))) as Box<dyn Piece>)
                    .collect::<Vec<Box<dyn Piece>>>()
            });

        let rest = HashMap::from([(Color::White, 0), (Color::Black, 7)])
            .into_iter()
            .flat_map(|(color, y)| {
                vec![
                    Box::new(Rook::new(color, Position::new(0, y))) as Box<dyn Piece>,
                    Box::new(Rook::new(color, Position::new(7, y))),
                    Box::new(Knight::new(color, Position::new(1, y))),
                    Box::new(Knight::new(color, Position::new(6, y))),
                    Box::new(Bishop::new(color, Position::new(2, y))),
                    Box::new(Bishop::new(color, Position::new(5, y))),
                    Box::new(Queen::new(color, Position::new(3, y))),
                    Box::new(King::new(color, Position::new(4, y))),
                ]
            });

        let pieces: Vec<Box<dyn Piece>> = pawns.chain(rest).collect();

        Self { pieces }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let horizontal_bar = "\u{2500}";
        let three_horizontal_bars =
            format!("{}{}{}", horizontal_bar, horizontal_bar, horizontal_bar);
        let top_left_corner = "\u{250C}";
        let top_right_corner = "\u{2510}";
        let bottom_left_corner = "\u{2514}";
        let bottom_right_corner = "\u{2518}";
        let dark_box = "\u{2580}";
        let space = " ";
        let vertical_bar = "\u{2502}";

        let indexed: HashMap<XY, &Box<dyn Piece>> = self
            .pieces
            .iter()
            .map(|piece| {
                let xy = piece.position().to_xy();
                (xy, piece)
            })
            .collect();

        let mut top_row = top_left_corner.to_owned();
        top_row.push_str(
            &(0..8)
                .map(|_| three_horizontal_bars.clone())
                .collect::<Vec<String>>()
                .join(horizontal_bar),
        );
        top_row.push_str(top_right_corner);

        let mut bottom_row = bottom_left_corner.to_owned();
        bottom_row.push_str(
            &(0..8)
                .map(|_| three_horizontal_bars.clone())
                .collect::<Vec<String>>()
                .join(horizontal_bar),
        );
        bottom_row.push_str(bottom_right_corner);

        let mut rows = Vec::with_capacity(8);

        for y in (0..8).rev() {
            let mut row = Vec::with_capacity(8);

            for x in 0..8 {
                let el = match indexed.get(&XY::OnBoard(x, y)) {
                    Some(piece) => piece.to_string(),
                    None => {
                        if (x + y) & 1 == 0 {
                            dark_box.to_string()
                        } else {
                            space.to_string()
                        }
                    }
                };

                row.push(el);
            }

            let mut row_string = String::new();
            row_string.push_str(vertical_bar);
            row_string.push(' ');
            row_string.push_str(&row.join(&format!(" {} ", vertical_bar)));
            row_string.push(' ');
            row_string.push_str(vertical_bar);

            rows.push(row_string)
        }

        let mut all = vec![top_row];
        all.extend_from_slice(&rows);
        all.push(bottom_row);
        let out = all.join("\n");

        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let expected = r#"
┌───────────────────────────────┐
│ ♜ │ ♞ │ ♝ │ ♛ │ ♚ │ ♝ │ ♞ │ ♜ │
│ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │
│   │ ▀ │   │ ▀ │   │ ▀ │   │ ▀ │
│ ▀ │   │ ▀ │   │ ▀ │   │ ▀ │   │
│   │ ▀ │   │ ▀ │   │ ▀ │   │ ▀ │
│ ▀ │   │ ▀ │   │ ▀ │   │ ▀ │   │
│ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │
│ ♖ │ ♘ │ ♗ │ ♕ │ ♔ │ ♗ │ ♘ │ ♖ │
└───────────────────────────────┘
"#
        .trim();

        let board = Board::default();

        assert_eq!(board.to_string(), expected);
    }
}
