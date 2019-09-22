use std::char;
use std::cmp;
use std::convert::TryFrom;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mfm = MinefieldMatrix::new_from_str_minefield(minefield);
    mfm.into()
}

#[derive(Clone, PartialEq)]
enum Square {
    Mine,
    CountMine(u32),
    NotYetSet,
}

#[derive(PartialEq)]
struct MineError;

impl TryFrom<char> for Square {
    type Error = MineError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '*' => Ok(Square::Mine),
            ' ' => Ok(Square::NotYetSet),
            _ => value.to_digit(10).ok_or(MineError).map(Square::CountMine),
        }
    }
}

impl From<Square> for char {
    fn from(s: Square) -> char {
        match s {
            Square::Mine => '*',
            Square::NotYetSet => ' ',
            Square::CountMine(i) => char::from_digit(i, 10).unwrap(),
        }
    }
}

#[derive(Clone)]
struct Line {
    squares: Vec<Square>,
}

impl Line {
    fn new(col_count: usize) -> Self {
        Self {
            squares: vec![Square::NotYetSet; col_count],
        }
    }
}

impl TryFrom<String> for Line {
    type Error = MineError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value
            .chars()
            .map(Square::try_from)
            .collect::<Result<Vec<Square>, MineError>>()
            .map(|squares| Line { squares })
    }
}

impl From<Line> for String {
    fn from(l: Line) -> String {
        l.squares.into_iter().map(|sq| char::from(sq)).collect()
    }
}

struct MinefieldMatrix {
    lines: Vec<Line>,
}

impl MinefieldMatrix {
    fn new(line_count: usize, col_count: usize) -> Self {
        Self {
            lines: vec![Line::new(col_count); line_count],
        }
    }

    fn new_from_str_minefield(minefield: &[&str]) -> Self {
        let (line_count, col_count) = match minefield.len() {
            0 => (0, 0),
            _ => (minefield.len(), minefield[0].chars().count()),
        };
        let mut mfm = Self::new(line_count, col_count);

        // update it with the supplied minefield
        minefield.iter().enumerate().for_each(|(line_num, line)| {
            line.char_indices().for_each(|(col_num, sq)| {
                if Square::try_from(sq) == Ok(Square::Mine) {
                    mfm.update_surrounding_squares(line_num, col_num);
                }
            })
        });
        mfm
    }

    fn update_surrounding_squares(&mut self, line: usize, col: usize) {
        self.lines[line].squares[col] = Square::Mine;
        for l in line.saturating_sub(1)..=cmp::min(line + 1, self.lines.len() - 1) {
            for c in col.saturating_sub(1)..=cmp::min(col + 1, &self.lines[l].squares.len() - 1) {
                self.lines[l].squares[c] = match &self.lines[l].squares[c] {
                    Square::Mine => Square::Mine,
                    Square::NotYetSet => Square::CountMine(1),
                    Square::CountMine(i) => Square::CountMine(i + 1),
                }
            }
        }
    }
}

impl From<MinefieldMatrix> for Vec<String> {
    fn from(mm: MinefieldMatrix) -> Vec<String> {
        mm.lines.into_iter().map(|l| String::from(l)).collect()
    }
}
