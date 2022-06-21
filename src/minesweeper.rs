use rand::prelude::*;
use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(usize),
}
#[derive(Debug, Clone)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    lost: bool,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);

                if !self.open_fields.contains(&pos) {
                    if self.lost && self.mines.contains(&pos) {
                        f.write_str("💣 ")?;
                    } else if self.flagged_fields.contains(&pos) {
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("🟪 ")?;
                    }
                } else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    let mine_count = self.neighboring_mines(pos);

                    if mine_count > 0 {
                        write!(f, " {} ", mine_count)?;
                    } else {
                        f.write_str("⬜ ")?;
                    }
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();
                let mut rng = thread_rng();
                while mines.len() < mine_count {
                    let x = rng.gen_range(0..width);
                    let y = rng.gen_range(0..height);
                    mines.insert((x, y));
                }
                mines
            },
            flagged_fields: HashSet::new(),
            lost: false,
        }
    }
    pub fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }
    pub fn neighboring_mines(&self, pos: Position) -> usize {
        self.iter_neighbors(pos)
            .filter(|pos| self.mines.contains(pos))
            .count()
    }
    pub fn open(&mut self, position: Position) -> Option<OpenResult> {
        if self.flagged_fields.contains(&position) {
            return None;
        }
        self.open_fields.insert(position);
        let is_mine = self.mines.contains(&position);
        if is_mine {
            Some(OpenResult::Mine)
        } else {
            Some(OpenResult::NoMine(0))
        }
    }
    pub fn toggle_flag(&mut self, pos: Position) {
        if self.lost || self.open_fields.contains(&pos) {
            return;
        }
        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);
        }
    }
    pub fn reset(&mut self) {
        *self = Minesweeper::new(10,10,5);
    }
}

#[cfg(test)]
mod tests {
    use super::Minesweeper;

    #[test]

    fn test() {
        let mut minesweeper = Minesweeper::new(10, 10, 5);
        minesweeper.open((5, 5));
        minesweeper.toggle_flag((6, 6));
        println!("{}", minesweeper)
    }
}
