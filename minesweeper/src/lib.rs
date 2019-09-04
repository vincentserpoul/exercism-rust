use std::cmp;
use std::fmt;
use std::str::FromStr;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mfm = MinefieldMatrix::new_from_str_minefield(minefield);
    mfm.lines.iter().map(|l| format!("{}", l)).collect()
}

#[derive(Clone, PartialEq)]
enum Square {
    Mine,
    CountMine(u32),
    NotYetSet,
}

impl FromStr for Square {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "*" => Ok(Square::Mine),
            " " => Ok(Square::NotYetSet),
            i => {
                if let Ok(j) = i.parse::<u32>() {
                    Ok(Square::CountMine(j))
                } else {
                    Err(())
                }
            }
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Square::Mine => write!(f, "*"),
            Square::CountMine(i) => write!(f, "{}", i),
            Square::NotYetSet => write!(f, " "),
        }
    }
}

#[derive(Clone)]
struct Line {
    squares: Vec<Square>,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            &self
                .squares
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .concat()
        )
    }
}

impl FromStr for Line {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let squares = s
            .chars()
            .flat_map(|sq| Square::from_str(sq.to_string().as_str()))
            .collect::<Vec<Square>>();
        Ok(Self { squares })
    }
}

impl Line {
    pub fn new(col_count: usize) -> Self {
        Self {
            squares: vec![Square::NotYetSet; col_count],
        }
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

    pub fn new_from_str_minefield(minefield: &[&str]) -> Self {
        // create new minefield
        let line_count: usize = minefield.len();
        let col_count: usize = if line_count > 0 {
            minefield[0].chars().count()
        } else {
            0
        };
        let mut mfm = Self::new(line_count, col_count);

        // update it with the supplied minefield
        minefield.iter().enumerate().for_each(|(line_num, line)| {
            line.chars().enumerate().for_each(|(col_num, sq)| {
                if Square::from_str(sq.to_string().as_str()) == Ok(Square::Mine) {
                    mfm.update_surrounding_squares(line_num, col_num);
                }
            })
        });
        mfm
    }

    pub fn update_surrounding_squares(&mut self, line: usize, col: usize) {
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
