#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Position {
    repr: Repr,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Repr {
    OffBoard,
    OnBoard(u8),
}

impl From<(u8, u8)> for Position {
    fn from(xy: (u8, u8)) -> Self {
        Self::new(xy.0, xy.1)
    }
}

impl Position {
    pub const fn new(x: u8, y: u8) -> Self {
        if x > 7 || y > 7 {
            return Self {
                repr: Repr::OffBoard,
            };
        }

        let repr = y * 8 + x;

        Self {
            repr: Repr::OnBoard(repr),
        }
    }

    pub const fn to_xy(self) -> XY {
        match self.repr {
            Repr::OffBoard => XY::OffBoard,
            Repr::OnBoard(repr) => {
                let x = repr % 8;
                let y = repr / 8;
                XY::OnBoard(x, y)
            }
        }
    }

    pub const fn up(&self) -> Self {
        match self.repr {
            Repr::OffBoard => *self,
            Repr::OnBoard(repr) => {
                if repr + 8 > 63 {
                    Position::off_board()
                } else {
                    Position {
                        repr: Repr::OnBoard(repr + 8),
                    }
                }
            }
        }
    }

    pub const fn down(&self) -> Self {
        match self.repr {
            Repr::OffBoard => *self,
            Repr::OnBoard(repr) => {
                if let Some(new_repr) = repr.checked_sub(8) {
                    Position {
                        repr: Repr::OnBoard(new_repr),
                    }
                } else {
                    Position::off_board()
                }
            }
        }
    }

    pub const fn left(&self) -> Self {
        match self.repr {
            Repr::OffBoard => *self,
            Repr::OnBoard(repr) => {
                if let Some(possible_repr) = repr.checked_sub(1) {
                    if crosses_row_boundary(repr, possible_repr) {
                        Position::off_board()
                    } else {
                        Position {
                            repr: Repr::OnBoard(possible_repr),
                        }
                    }
                } else {
                    Position::off_board()
                }
            }
        }
    }

    pub const fn right(&self) -> Self {
        match self.repr {
            Repr::OffBoard => *self,
            Repr::OnBoard(repr) => {
                if crosses_row_boundary(repr, repr + 1) || repr + 1 > 63 {
                    Position::off_board()
                } else {
                    Position {
                        repr: Repr::OnBoard(repr + 1),
                    }
                }
            }
        }
    }

    #[inline]
    pub const fn up_right(&self) -> Self {
        self.up().right()
    }

    #[inline]
    pub const fn up_left(&self) -> Self {
        self.up().left()
    }

    #[inline]
    pub const fn down_right(&self) -> Self {
        self.down().right()
    }

    #[inline]
    pub const fn down_left(&self) -> Self {
        self.down().left()
    }

    pub fn stream(self, direction: MoveDirection) -> impl Iterator<Item = Position> {
        let mut position_state = self;

        std::iter::from_fn(move || {
            position_state = match direction {
                MoveDirection::Up => position_state.up(),
                MoveDirection::Down => position_state.down(),
                MoveDirection::Left => position_state.left(),
                MoveDirection::Right => position_state.right(),
                MoveDirection::UpRight => position_state.up_right(),
                MoveDirection::UpLeft => position_state.up_left(),
                MoveDirection::DownRight => position_state.down_right(),
                MoveDirection::DownLeft => position_state.down_left(),
            };

            if position_state.is_on_board() {
                Some(position_state)
            } else {
                None
            }
        })
    }

    pub fn compose<const N: usize>(moves: [MoveDirection; N]) -> impl Fn(Position) -> Position {
        move |starting_position| -> Position {
            moves.iter().fold(
                starting_position,
                |position, move_direction| match move_direction {
                    MoveDirection::Up => position.up(),
                    MoveDirection::Down => position.down(),
                    MoveDirection::Left => position.left(),
                    MoveDirection::Right => position.right(),
                    MoveDirection::UpRight => position.up_right(),
                    MoveDirection::UpLeft => position.up_left(),
                    MoveDirection::DownRight => position.down_right(),
                    MoveDirection::DownLeft => position.down_left(),
                },
            )
        }
    }

    #[inline]
    pub const fn is_on_board(&self) -> bool {
        match self.repr {
            Repr::OffBoard => false,
            Repr::OnBoard(_) => true,
        }
    }

    #[inline]
    const fn off_board() -> Self {
        Position {
            repr: Repr::OffBoard,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum XY {
    OffBoard,
    OnBoard(u8, u8),
}

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

#[inline]
const fn crosses_row_boundary(previous: u8, next: u8) -> bool {
    previous / 8 != next / 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_xy() {
        for y in 0..7 {
            for x in 0..7 {
                let position: Position = (x, y).into();
                assert_eq!(XY::OnBoard(x, y), position.to_xy())
            }
        }

        for y in 8..u8::MAX {
            for x in 8..u8::MAX {
                let position: Position = (x, y).into();
                assert_eq!(XY::OffBoard, position.to_xy())
            }
        }
    }

    #[test]
    fn up() {
        assert_eq!(Position::new(4, 0).up(), (4, 1).into());

        for x in 0u8..8 {
            assert_eq!(Position::new(x, 7).up().to_xy(), XY::OffBoard);
        }
    }

    #[test]
    fn down() {
        assert_eq!(Position::new(4, 4).down(), (4, 3).into());

        for x in 0..8 {
            println!("{}", x);
            assert_eq!(Position::new(x, 0).down().to_xy(), XY::OffBoard)
        }
    }

    #[test]
    fn left() {
        assert_eq!(Position::new(4, 4).left(), (3, 4).into());

        for y in 0..8 {
            assert_eq!(Position::new(0, y).left().to_xy(), XY::OffBoard);
        }
    }

    #[test]
    fn right() {
        assert_eq!(Position::new(4, 4).right(), (5, 4).into());

        for y in 0..8 {
            assert_eq!(Position::new(7, y).right().to_xy(), XY::OffBoard)
        }
    }

    #[test]
    fn stream() {
        let expected: Vec<_> = (1..8).map(|y| (4, y).into()).collect();
        eprintln!("{:?}", expected);

        let test = Position::new(4, 0)
            .stream(MoveDirection::Up)
            .collect::<Vec<Position>>();

        assert_eq!(test, expected);

        let expected: Vec<_> = (4..8).map(|x| (x, x).into()).collect();

        assert_eq!(
            Position::new(3, 3)
                .stream(MoveDirection::UpRight)
                .collect::<Vec<Position>>(),
            expected
        )
    }
}
